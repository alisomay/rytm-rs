extern crate proc_macro;

mod machine_parameters;
mod parameter_range;
mod util;

use machine_parameters::*;
use parameter_range::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Error as SynError, ItemFn,
};

/// A proc macro to apply range validation for function parameters used in `rytm-rs`.
///
/// Please check `rytm-rs` source code for exhaustive usage examples.
///
/// # Example
///
/// ```
/// #[parameter_range(range = "arg_name:0..=127")]
/// #[parameter_range(range = "arg_name:0..=127:u16")]
/// #[parameter_range(range = "arg_name[opt]:0..=127")]
/// #[parameter_range(range = "arg_name:0..=127", range = "another_arg_name:0.4..=127.8")]
/// ```
#[proc_macro_attribute]
pub fn parameter_range(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let args: Punctuated<RangeArg, Comma> =
        parse_macro_input!(args with Punctuated::parse_terminated);

    let mut checks = Vec::new();
    for arg in args {
        let param_name_ident = format_ident!("{}", arg.param_name);
        let is_inclusive = arg.range_expr.contains("..=");
        let range_parts: Vec<&str> = if is_inclusive {
            arg.range_expr.split("..=").collect()
        } else {
            arg.range_expr.split("..").collect()
        };

        if range_parts.len() != 2 {
            return SynError::new_spanned(
                &param_name_ident,
                "Invalid range expression format. Expected format: `start..=end` or `start..end`",
            )
            .to_compile_error()
            .into();
        }

        let start = range_parts[0];
        let end = range_parts[1];

        let type_annotation = arg.param_type.map_or_else(
            || quote! { _ },
            |ty| {
                let ty = syn::parse_str::<syn::Type>(&ty).expect("Invalid type annotation");
                quote! { #ty }
            },
        );

        let range_check = if is_inclusive {
            if arg.is_optional {
                quote! {
                    let ___start: #type_annotation = #start.parse().expect("Invalid range start");
                    let ___end: #type_annotation = #end.parse().expect("Invalid range end");


                    if let Some(false) = #param_name_ident.as_ref().map(|&x| x >= ___start && x <= ___end) {
                        return Err(RytmError::Parameter(ParameterError::Range {
                            value: #param_name_ident.unwrap().to_string(),
                            parameter_name: stringify!(#param_name_ident).to_string(),
                        }));
                    }
                }
            } else {
                quote! {
                    let ___start: #type_annotation = #start.parse().expect("Invalid range start");
                    let ___end: #type_annotation = #end.parse().expect("Invalid range end");

                    if !(___start..=___end).contains(&#param_name_ident) {
                        return Err(RytmError::Parameter(ParameterError::Range {
                            value: #param_name_ident.to_string(),
                            parameter_name: stringify!(#param_name_ident).to_string(),
                        }));
                    }
                }
            }
        } else if arg.is_optional {
            quote! {
                let ___start: #type_annotation = #start.parse().expect("Invalid range start");
                let ___end: #type_annotation = #end.parse().expect("Invalid range end");


                if let Some(false) = #param_name_ident.as_ref().map(|&x| x >= ___start && x < ___end) {
                    return Err(RytmError::Parameter(ParameterError::Range {
                        value: #param_name_ident.unwrap().to_string(),
                        parameter_name: stringify!(#param_name_ident).to_string(),
                    }));
                }
            }
        } else {
            quote! {
                let ___start: #type_annotation = #start.parse().expect("Invalid range start");
                let ___end: #type_annotation = #end.parse().expect("Invalid range end");

                if !(___start..___end).contains(&#param_name_ident) {
                    return Err(RytmError::Parameter(ParameterError::Range {
                        value: #param_name_ident.to_string(),
                        parameter_name: stringify!(#param_name_ident).to_string(),
                    }));
                }
            }
        };

        checks.push(range_check);
    }

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input_fn;

    let result = quote! {
        #(#attrs)* #vis #sig {
            #(#checks)*
            #block
        }
    };

    result.into()
}

/// A proc macro to implement normal and parameter lock getter, setter and clearer methods mor machine parameter structs.
///
/// `#<number>` annotations denote synth parameter indices used internally.
///
/// The macro also generates a `apply_to_raw_sound_values` method which is used to apply the parameter values to the raw sound struct.
///
/// Please check `rytm-rs` source code for exhaustive usage examples.
///
/// # Example
///
/// ```
/// #[machine_parameters(
///     lev: "0..=127" #1,
///     tun: "-32.0..=32.0" #2,
///     dec: "0..=127" #3,
///     hld: "0..=127" #4,
///     swt: "0..=127" #5,
///     swd: "0..=127" #6,
///     wav: "0..=2" #7,
///     tra: "0..=127" #8,
/// )]
/// ```
#[proc_macro_attribute]
pub fn machine_parameters(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParameterArgs);
    let input_struct = parse_macro_input!(input as DeriveInput);
    let struct_name = input_struct.ident.clone();

    let methods = args.0.iter().map(|arg| {
        // let (param_type, return_type) = determine_types(&arg.range);
        let setter_and_plock_methods =
            generate_setter_and_plock_methods_with_range_check(arg, &struct_name);
        let getter = generate_getter(arg, &struct_name);
        quote! { #setter_and_plock_methods #getter }
    });

    let apply_to_raw_sound_values_inner =
        args.0.iter().map(generate_apply_to_raw_sound_values_inner);

    let result = quote! {
        #input_struct

        impl #struct_name {
            pub(crate) fn apply_to_raw_sound_values(&self, raw_sound: &mut ar_sound_t) {
                #(#apply_to_raw_sound_values_inner)*
            }
        }

        #(#methods)*
    };

    result.into()
}

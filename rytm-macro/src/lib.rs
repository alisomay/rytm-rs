extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_str,
    punctuated::Punctuated,
    token::Comma,
    DeriveInput, Error as SynError, Ident, ItemFn, Lit, LitInt, LitStr, Meta, Token,
};

// Parameter Range

struct RangeArg {
    param_name: String,
    range_expr: String,
    param_type: Option<String>,
}

impl Parse for RangeArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let nv: Meta = input.parse()?;
        if let Meta::NameValue(nv) = nv {
            if nv.path.is_ident("range") {
                match &nv.value {
                    syn::Expr::Lit(expr_lit) => {
                        if let Lit::Str(lit) = &expr_lit.lit {
                            let value = lit.value();
                            let parts: Vec<_> = value.split(':').collect();

                            // Check for two or three parts
                            if parts.len() < 2 || parts.len() > 3 {
                                return Err(SynError::new_spanned(
                                    lit,
                                    "Expected format: `param_name:range` or `param_name:range:type`",
                                ));
                            }

                            let param_type = if parts.len() == 3 {
                                Some(parts[2].trim().to_string())
                            } else {
                                None
                            };

                            Ok(RangeArg {
                                param_name: parts[0].trim().to_string(),
                                range_expr: parts[1].trim().to_string(),
                                param_type,
                            })
                        } else {
                            Err(SynError::new_spanned(
                                &expr_lit.lit,
                                "The range value must be a string",
                            ))
                        }
                    }
                    _ => Err(SynError::new_spanned(
                        &nv.value,
                        "Expected a string literal",
                    )),
                }
            } else {
                Err(SynError::new_spanned(nv.path, "Expected `range`"))
            }
        } else {
            Err(SynError::new_spanned(nv, "Expected name-value pair"))
        }
    }
}

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

// Machine Parameters

struct ParameterArgs(Vec<ParameterArg>);

impl Parse for ParameterArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let punctuated = Punctuated::<ParameterArg, Token![,]>::parse_terminated(input)?;
        Ok(ParameterArgs(punctuated.into_iter().collect()))
    }
}

#[derive(Debug, Default)]
struct ParameterArg {
    name: String,
    range: String,
    param_type: Option<String>, // Optional field for type
    syn_param_type: usize,
}

impl Parse for ParameterArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        input.parse::<Token![:]>()?;
        let range_lit: LitStr = input.parse()?;
        let range = range_lit.value();

        // Optional: Parse the type annotation if provided
        let param_type = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let type_lit: LitStr = input.parse()?;
            Some(type_lit.value())
        } else {
            None
        };

        input.parse::<Token![#]>()?;
        let type_lit_int: LitInt = input.parse()?;
        let syn_param_type: usize = type_lit_int
            .base10_parse()
            .expect("Syn parameter type number needs to be provided. ");

        Ok(ParameterArg {
            name: name.to_string(),
            range,
            param_type,
            syn_param_type,
        })
    }
}

fn generate_getter(parameter: &ParameterArg, struct_name: &Ident) -> proc_macro2::TokenStream {
    let param_name = &parameter.name;
    let param_ident = parse_str::<Ident>(param_name).unwrap();
    let getter_fn_name = format_ident!("get_{}", param_name);
    let return_type = determine_return_type(&parameter.range);
    let range = &parameter.range;

    let doc_comment_setter = format!(
        " Gets the `{}` parameter.\n\n Range: `{}`",
        param_name, range
    );

    quote! {
        impl #struct_name {
            #[doc = #doc_comment_setter]
            pub fn #getter_fn_name(&self) -> #return_type {
                self.#param_ident as #return_type
            }
        }
    }
}

fn generate_range_check_logic(parameter: &ParameterArg) -> proc_macro2::TokenStream {
    let param_ident = format_ident!("{}", parameter.name);
    let is_inclusive = parameter.range.contains("..=");
    let range_parts: Vec<&str> = if is_inclusive {
        parameter.range.split("..=").collect()
    } else {
        parameter.range.split("..").collect()
    };

    let start = range_parts[0];
    let end = range_parts[1];

    // Infer the type for parsing based on the format of the range string
    let parse_type = if let Some(ty) = &parameter.param_type {
        quote! { #ty }
    } else {
        infer_type_from_range(start, end)
    };

    // Generate range check logic
    if is_inclusive {
        quote! {
            let start: #parse_type = #start.parse().expect("Invalid range start");
            let end: #parse_type = #end.parse().expect("Invalid range end");

            if !(start..=end).contains(&#param_ident) {
                return Err(RytmError::Parameter(ParameterError::Range {
                    parameter_name: stringify!(#param_ident).to_string(),
                    value: #param_ident.to_string(),
                }));
            }
        }
    } else {
        quote! {
            let start: #parse_type = #start.parse().expect("Invalid range start");
            let end: #parse_type = #end.parse().expect("Invalid range end");

            if !(start..end).contains(&#param_ident) {
                return Err(RytmError::Parameter(ParameterError::Range {
                    parameter_name: stringify!(#param_ident).to_string(),
                    value: #param_ident.to_string(),
                }));
            }
        }
    }
}

// Helper function to infer type from range
fn infer_type_from_range(start: &str, end: &str) -> proc_macro2::TokenStream {
    if start.contains('.') || end.contains('.') {
        quote! { f32 }
    } else if start.starts_with('-') || end.starts_with('-') {
        quote! { isize }
    } else {
        quote! { usize }
    }
}

fn generate_setter_with_range_check(
    parameter: &ParameterArg,
    struct_name: &Ident,
) -> proc_macro2::TokenStream {
    let param_name = &parameter.name;
    let param_ident = parse_str::<Ident>(param_name).unwrap();
    let setter_fn_name = format_ident!("set_{}", param_name);
    let range = &parameter.range;

    let (param_input_type, as_conversion_type) = determine_types(&parameter.range);
    let range_check = generate_range_check_logic(parameter);
    let doc_comment_setter = format!(
        " Sets the `{}` parameter.\n\n Range: `{}`",
        param_name, range
    );

    quote! {
        impl #struct_name {
            #[doc = #doc_comment_setter]
            pub fn #setter_fn_name(&mut self, #param_ident: #param_input_type) -> Result<(), RytmError> {
                #range_check
                self.#param_ident =  #param_ident as #as_conversion_type;
                Ok(())
            }
        }
    }
}

fn determine_types(range: &str) -> (syn::Type, syn::Type) {
    let parts: Vec<&str> = if range.contains("..=") {
        range.split("..=").collect()
    } else {
        range.split("..").collect()
    };

    let is_floating_point = parts.iter().any(|&p| p.contains('.'));
    if is_floating_point {
        // Floating point range
        (
            parse_str::<syn::Type>("f32").unwrap(),
            parse_str::<syn::Type>("f32").unwrap(),
        )
    } else if parts.iter().any(|&p| p.starts_with('-')) {
        // Range with negative values
        (
            parse_str::<syn::Type>("isize").unwrap(),
            parse_str::<syn::Type>("i8").unwrap(),
        )
    } else {
        // Default case
        (
            parse_str::<syn::Type>("usize").unwrap(),
            parse_str::<syn::Type>("u8").unwrap(),
        )
    }
}

fn determine_return_type(range: &str) -> syn::Type {
    let parts: Vec<&str> = if range.contains("..=") {
        range.split("..=").collect()
    } else {
        range.split("..").collect()
    };

    let is_floating_point = parts.iter().any(|&p| p.contains('.'));
    if is_floating_point {
        // For floating point ranges
        parse_str::<syn::Type>("f32").unwrap()
    } else if parts.iter().any(|&p| p.starts_with('-')) {
        // For ranges with negative values
        parse_str::<syn::Type>("isize").unwrap()
    } else {
        // Default case
        parse_str::<syn::Type>("usize").unwrap()
    }
}

fn generate_apply_to_raw_sound_values_inner(
    parameter: &ParameterArg,
    // struct_name: &Ident,
) -> proc_macro2::TokenStream {
    let param_name = &parameter.name;
    let param_ident = parse_str::<Ident>(param_name).unwrap();
    let range = &parameter.range;

    let parts: Vec<&str> = if range.contains("..=") {
        range.split("..=").collect()
    } else {
        range.split("..").collect()
    };

    let syn_param = format_ident!("synth_param_{}", parameter.syn_param_type);
    let is_floating_point = parts.iter().any(|&p| p.contains('.'));
    if is_floating_point {
        let input_min = parts[0].parse::<f32>().expect("");
        let input_max = parts[1].parse::<f32>().expect("");
        let (output_min, output_max) = get_u16_min_max_from_float_range(input_min, input_max);

        quote! {
            raw_sound.#syn_param = to_s_u16_t_union_a(scale_generic(
                self.#param_ident,
                #input_min,
                #input_max,
                #output_min,
                #output_max,
                |#param_ident: f32| #param_ident.round() as u16,
            ));
        }
    } else if parts.iter().any(|&p| p.starts_with('-')) {
        // We assume that the u8 range is always between 0..127
        // If this changes there should be a more robust implementation.

        // Range with negative values
        quote! {
            raw_sound.#syn_param = to_s_u16_t_union_a(((i8_to_u8_midpoint_of_u8_input_range(self.#param_ident, 0, 127) as u16)) << 8);
        }
    } else {
        quote! {
            raw_sound.#syn_param = to_s_u16_t_union_a((self.#param_ident as u16) << 8);
        }
        // Default case
    }
}

#[proc_macro_attribute]
pub fn machine_parameters(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ParameterArgs);
    let input_struct = parse_macro_input!(input as DeriveInput);
    let struct_name = input_struct.ident.clone();

    let methods = args.0.iter().map(|arg| {
        // let (param_type, return_type) = determine_types(&arg.range);
        let setter = generate_setter_with_range_check(arg, &struct_name);
        let getter = generate_getter(arg, &struct_name);
        quote! { #setter #getter }
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

// Helper function to decode synth parameter float minus plus scaling.
fn get_u16_min_max_from_float_range(min: f32, max: f32) -> (u16, u16) {
    // Given example ranges
    let example_float_min = -32.0;
    let example_float_max = 32.0;
    let example_u16_min = 8192u16;
    let example_u16_max = 24576u16;

    // Calculate the scale factor based on the example
    let example_float_range = example_float_max - example_float_min;
    let example_u16_range = example_u16_max as f32 - example_u16_min as f32;
    let scale_factor = example_u16_range / example_float_range;

    // Apply the scale factor to the given range
    let scaled_min = ((min - example_float_min) * scale_factor) as u16 + example_u16_min;
    let scaled_max = ((max - example_float_min) * scale_factor) as u16 + example_u16_min;

    (scaled_min, scaled_max)
}

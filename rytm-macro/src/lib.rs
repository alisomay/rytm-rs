extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Error as SynError, ItemFn, Lit, Meta,
};

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
                let start: #type_annotation = #start.parse().expect("Invalid range start");
                let end: #type_annotation = #end.parse().expect("Invalid range end");

                if !(start..=end).contains(&#param_name_ident) {
                    return Err(RytmError::Parameter(ParameterError::Range {
                        value: #param_name_ident.to_string(),
                        parameter_name: stringify!(#param_name_ident).to_string(),
                    }));
                }
            }
        } else {
            quote! {
                let start: #type_annotation = #start.parse().expect("Invalid range start");
                let end: #type_annotation = #end.parse().expect("Invalid range end");

                if !(start..end).contains(&#param_name_ident) {
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

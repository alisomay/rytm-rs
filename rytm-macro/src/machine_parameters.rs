use super::util::*;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_str,
    punctuated::Punctuated,
    Ident, LitInt, LitStr, Token,
};

pub struct ParameterArgs(pub Vec<ParameterArg>);

impl Parse for ParameterArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let punctuated = Punctuated::<ParameterArg, Token![,]>::parse_terminated(input)?;
        Ok(ParameterArgs(punctuated.into_iter().collect()))
    }
}

#[derive(Debug, Default)]
pub struct ParameterArg {
    pub name: String,
    pub range: String,
    pub param_type: Option<String>, // Optional field for type
    pub syn_param_type: usize,
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

pub fn generate_getter(parameter: &ParameterArg, struct_name: &Ident) -> proc_macro2::TokenStream {
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

pub fn generate_range_check_logic(parameter: &ParameterArg) -> proc_macro2::TokenStream {
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

pub fn generate_setter_with_range_check(
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

pub fn generate_apply_to_raw_sound_values_inner(
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

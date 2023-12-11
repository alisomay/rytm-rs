use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Lit, Meta,
};

pub struct RangeArg {
    pub param_name: String,
    pub range_expr: String,
    pub param_type: Option<String>,
    pub is_optional: bool,
}

fn extract_bracket_contents(s: &mut String) -> Option<String> {
    let (start, end) = (s.find('[')?, s.rfind(']')?);
    if start < end {
        let contents = s[start + 1..end].to_string();
        s.replace_range(start..=end, "");
        Some(contents)
    } else {
        None
    }
}

impl Parse for RangeArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let nv: Meta = input.parse()?;
        if let Meta::NameValue(nv) = nv {
            if nv.path.is_ident("range") {
                match &nv.value {
                    syn::Expr::Lit(expr_lit) => {
                        if let Lit::Str(lit) = &expr_lit.lit {
                            let mut value = lit.value();
                            // There can be exmaple something[opt] or something before the :

                            let mut is_optional = false;
                            if let Some(content) = extract_bracket_contents(&mut value) {
                                if content != "opt" {
                                    return Err(SynError::new_spanned(
                                        lit,
                                        "only `opt` is supported as an option",
                                    ));
                                }
                                is_optional = true;
                            }

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
                                is_optional,
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

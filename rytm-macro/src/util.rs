use quote::quote;
use syn::parse_str;

// Helper function to decode synth parameter float minus plus scaling.
pub fn get_u16_min_max_from_float_range(min: f32, max: f32) -> (u16, u16) {
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

pub fn determine_return_type(range: &str) -> syn::Type {
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

pub fn determine_types(range: &str) -> (syn::Type, syn::Type) {
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

// Helper function to infer type from range
pub fn infer_type_from_range(start: &str, end: &str) -> proc_macro2::TokenStream {
    if start.contains('.') || end.contains('.') {
        quote! { f32 }
    } else if start.starts_with('-') || end.starts_with('-') {
        quote! { isize }
    } else {
        quote! { usize }
    }
}

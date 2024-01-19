use proc_macro::TokenStream;
use std::collections::HashMap;
use std::fs;
use proc_macro2::Span;
use syn::parse_macro_input;
use crate::input::ConfigInput;
use crate::output::generate_config_struct;
#[cfg(feature = "struct")]
use syn::DeriveInput;

mod output;
mod input;
#[cfg(feature = "struct")]
mod struct_output;

fn find_yaml_values(input: &ConfigInput) -> Result<HashMap<String, String>, syn::Error> {
    let file_name = if let Some(path) = &input.path {
        path.to_string()
    } else {
        "./configuration/config.yaml".to_string()
    };

    let file = fs::File::open(&file_name)
        .map_err(|_| {
            syn::Error::new(
                Span::call_site(),
                format!("could not find config with path {}", &file_name),
            )
        })?;
    Ok(serde_yaml::from_reader(file)
        .map_err(|e| {
            syn::Error::new(Span::call_site(), e.to_string())
        })?)
}

#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    let input: ConfigInput = parse_macro_input!(item);

    match find_yaml_values(&input) {
        Ok(values) => generate_config_struct(values).into(),
        Err(e) => e.into_compile_error().into()
    }
}

#[cfg(feature = "struct")]
#[proc_macro_attribute]
pub fn config_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ConfigInput = parse_macro_input!(attr);
    let ast: DeriveInput = parse_macro_input!(item);

    match find_yaml_values(&input) {
        Ok(values) => struct_output::generate_annotation_struct(ast, values, &input.exclude_from).into(),
        Err(e) => e.into_compile_error().into()
    }
}

use proc_macro::TokenStream;
use std::collections::HashMap;
use std::fs;
use proc_macro2::Span;

use syn::{parse_macro_input};

use crate::input::ConfigInput;
use crate::output::generate_config_struct;

mod output;
mod input;

fn find_yaml_values(input: ConfigInput) -> Result<HashMap<String, String>, syn::Error> {
    let file_name = input.path
        .unwrap_or_else(|| {
            "./configuration/config.yaml".to_string()
        });

    let file = fs::File::open(&file_name)
        .map_err(|err| {
            syn::Error::new(
                Span::call_site(),
                format!("could not read config with path {}: {}", &file_name, err)
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

    match find_yaml_values(input) {
        Ok(values) => generate_config_struct(values).into(),
        Err(e) => e.into_compile_error().into()
    }
}

#![doc = include_str!("../README.md")]

//! ## Documentation from lib.rs
//! Here is documentation placed directly within lib.rs...

use proc_macro::TokenStream;
use std::collections::HashMap;
use std::{fs};
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
    serde_yaml::from_reader(file)
        .map_err(|e| {
            syn::Error::new(Span::call_site(), e.to_string())
        })
}

/// This function-like macro will generate a struct called `Config`
/// which contains a HashMap<String,String> with all the yaml configuration properties.
/// The macro will also generate a `new` method for this struct,
/// which will add the key-values to the struct
#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    let input: ConfigInput = parse_macro_input!(item);

    match find_yaml_values(&input) {
        Ok(values) => generate_config_struct(values).into(),
        Err(e) => e.into_compile_error().into()
    }
}

/// This macro allows manipulation of an existing struct to serve as a 'config' struct
/// It will replace any existing fields with those present in the configuration.
/// It will also generate a `new` method to fill in the fields
/// (This macro has to be enabled using the 'struct' feature.)
///
/// ```rust
/// use config_macro::config_struct;
///
/// #[config_struct(path  = "./configuration/config.yaml")]
/// struct Example {}
///
/// // Example now has a new method
/// let e = Example::new();
///
/// // e now contains a 'user' field that we can access
/// println!("{}", e.user);
/// ```
///
/// We also generate a method to change the struct into a HashMap<String,String>
/// If you don't need this method, you can remove it with 'exclude = "from"'
///
#[cfg(any(feature = "struct",doc))]
#[proc_macro_attribute]
pub fn config_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ConfigInput = parse_macro_input!(attr);
    let ast: DeriveInput = parse_macro_input!(item);

    match find_yaml_values(&input) {
        Ok(values) => struct_output::generate_annotation_struct(ast, values, &input.exclude_from).into(),
        Err(e) => e.into_compile_error().into()
    }
}

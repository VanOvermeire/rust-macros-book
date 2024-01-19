use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::DeriveInput;

fn generate_inserts(yaml_values: HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values.iter().map(|v| {
        let key = v.0;
        let value = v.1;
        quote!(map.insert(#key.to_string(), #value.to_string());)
    }).collect()
}

pub fn generate_config_struct(yaml_values: HashMap<String, String>) -> TokenStream {
    let inserts = generate_inserts(yaml_values);

    quote! {
        pub struct Config(pub std::collections::HashMap<String,String>);

        impl Config {
            pub fn new() -> Self {
                let mut map = std::collections::HashMap::new();
                #(#inserts)*
                Config(map)
            }
        }
    }
}

fn generate_fields(yaml_values: &HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values.iter().map(|v| {
        let key = Ident::new(v.0, Span::call_site());
        quote! {
            pub #key: String
        }
    }).collect()
}

fn generate_inits(yaml_values: &HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values.iter().map(|v| {
        let key = Ident::new(v.0, Span::call_site());
        let value = v.1;
        quote! {
            #key: #value.to_string()
        }
    }).collect()
}

pub fn generate_annotation_struct(input: DeriveInput, yaml_values: HashMap<String, String>) -> TokenStream {
    let attributes = &input.attrs;
    let name = &input.ident;
    let fields = generate_fields(&yaml_values);
    let inits = generate_inits(&yaml_values);

    quote! {
        #(#attributes)*
        pub struct #name {
            #(#fields,)*
        }

        impl #name {
            pub fn new() -> Self {
                #name {
                    #(#inits,)*
                }
            }
        }
    }
}

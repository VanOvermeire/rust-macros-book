use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::DeriveInput;

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

fn generate_inserts_for_from(yaml_values: &HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values.iter().map(|v| {
        let key = v.0;
        let key_as_ident = Ident::new(key, Span::call_site());
        quote!(map.insert(#key.to_string(), value.#key_as_ident);)
    }).collect()
}

fn generate_from_method(name: &Ident, yaml_values: &HashMap<String, String>) -> TokenStream {
    let inserts = generate_inserts_for_from(yaml_values);

    quote! {
        impl From <#name> for std::collections::HashMap<String,String> {
            fn from(value: #name) -> Self {
                let mut map = std::collections::HashMap::new();
                #(#inserts)*
                map
            }
        }
    }
}

pub fn generate_annotation_struct(input: DeriveInput, yaml_values: HashMap<String, String>, exclude_from_method: &bool) -> TokenStream {
    let attributes = &input.attrs;
    let name = &input.ident;
    let fields = generate_fields(&yaml_values);
    let inits = generate_inits(&yaml_values);
    let from = if !exclude_from_method { generate_from_method(name, &yaml_values) } else { quote!() };

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

        #from
    }
}

mod fields;

use proc_macro2::{TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, DeriveInput, parse2};
use syn::Data::Struct;
use syn::DataStruct;
use syn::Fields::Named;
use syn::FieldsNamed;
use crate::fields::{builder_field_definitions, builder_init_values, builder_methods, optional_default_asserts, original_struct_setters};

const DEFAULTS_ATTRIBUTE_NAME: &str = "builder_defaults";

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(item).unwrap();
    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);
    let use_defaults = use_defaults(&ast.attrs);

    let fields = match ast.data {
        Struct(DataStruct { fields: Named(FieldsNamed { ref named, .. }), .. }) => named,
        _ => unimplemented!("only implemented for structs"),
    };
    let builder_fields = builder_field_definitions(fields);
    let builder_inits = builder_init_values(fields);
    let builder_methods = builder_methods(fields);
    let set_fields = original_struct_setters(fields, use_defaults);

    let default_assertions = if use_defaults {
        optional_default_asserts(fields)
    } else {
        vec![]
    };

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }
        impl #builder {
            #(#builder_methods)*

            pub fn build(self) -> #name {
                #name {
                    #(#set_fields,)*
                }
            }
        }
        impl #name {
            pub fn builder() -> #builder {
                #builder {
                    #(#builder_inits,)*
                }
            }
        }

        #(#default_assertions)*
    }
}

fn use_defaults(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attribute| attribute.path().is_ident(DEFAULTS_ATTRIBUTE_NAME))
}

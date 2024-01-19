mod fields;

use proc_macro2::{TokenStream};
use quote::{format_ident, quote};
use syn::DeriveInput;
use syn::Data::Struct;
use syn::DataStruct;
use syn::Fields::Named;
use syn::FieldsNamed;
use crate::fields::{
    builder_field_definitions,
    builder_init_values,
    builder_methods,
    original_struct_setters
};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);

    let fields = match ast.data {
        Struct(DataStruct { fields: Named(FieldsNamed { ref named, .. }), .. }) => named,
        _ => unimplemented!("only implemented for structs"),
    };
    let builder_fields = builder_field_definitions(fields);
    let builder_inits = builder_init_values(fields);
    let builder_methods = builder_methods(fields);
    let set_fields = original_struct_setters(fields);

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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_struct_for_one_field_struct_should_be_present_in_output() {
        let input = quote! {
            struct StructWithOneField {
                string_value: String,
            }
        };
        let expected = quote! {
            struct StructWithOneFieldBuilder {
                string_value: Option<String>,
            }

            impl StructWithOneFieldBuilder {
                pub fn string_value(mut self, input: String) -> Self {
                    self.string_value = Some(input);
                    self
                }

                pub fn build(self) -> StructWithOneField {
                    StructWithOneField {
                        string_value: self.string_value
                            .expect(concat!("field not set: " , "string_value")),

                    }
                }
            }

            impl StructWithOneField {
                pub fn builder() -> StructWithOneFieldBuilder {
                    StructWithOneFieldBuilder {
                        string_value: None,
                    }
                }
            }
        };

        let actual = create_builder(input);

        assert_eq!(actual.to_string(), expected.to_string());
    }
}

use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, DataStruct, FieldsNamed, Ident, Type};
use syn::__private::{Span, TokenStream2};
use syn::Data::Struct;
use syn::Fields::Named;

fn get_field_info(ast: &DeriveInput) -> Vec<(&Ident, &Type)> {
    match ast.data {
        Struct(
            DataStruct {
                fields: Named(
                    FieldsNamed {
                        ref named, ..
                    }), ..
            }
        ) => named,
        _ => unimplemented!(
            "only works for structs with named fields"
        ),
    }
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().take().unwrap();
            let type_name = &f.ty;

            (field_name, type_name)
        })
        .collect()
}

fn generated_methods(fields: &Vec<(&Ident, &Type)>) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|f| {
            let (field_name, type_name) = f;
            let method_name =
                Ident::new(
                &format!("get_{field_name}"),
                Span::call_site()
            );

            quote!(
                pub fn #method_name(&self) -> &#type_name {
                    &self.#field_name
                }
            )
        })
        .collect()
}

fn generate_private_fields(fields: &Vec<(&Ident, &Type)>) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|f| {
            let (field_name, type_name) = f;

            quote!(
                #field_name: #type_name
            )
        })
        .collect()
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let fields = get_field_info(&ast);
    let output_fields = generate_private_fields(&fields);
    let methods = generated_methods(&fields);

    quote!(
        pub struct #name {
            #(#output_fields,)*
        }

        impl #name {
            pub fn new() -> Self {
                Example {
                    string_value: "value".to_string(),
                    number_value: 2,
                }
            }

            #(#methods)*
        }
    ).into()
}


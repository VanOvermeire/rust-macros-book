use proc_macro2::{TokenStream};
use quote::quote;
use syn::{Meta, Field, Ident, LitStr, Type};
use syn::__private::TokenStream2;
use syn::punctuated::{Punctuated};
use syn::token::Comma;

pub fn builder_methods(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields.iter()
        .map(|f| {
            let (field_name, field_type) = get_name_and_type(f);
            let attr = extract_attribute_from_field(f, "rename")
                // alternative if you decide to forgo quotation marks in the list
                // .map(|a| {
                //     let mut content = None;
                //
                //     a.parse_nested_meta(|m| {
                //         let i = &m.path.segments.first().unwrap().ident;
                //         content = Some(Ident::new(&i.to_string(), i.span()));
                //         Ok(())
                //     }).unwrap();
                //
                //     content.unwrap()
                // })
                .map(|a| &a.meta)
                .map(|m| {
                    match m {
                        Meta::List(nested) => {
                            // &nested.tokens // if you decide to forgo quotation marks in the list
                            let a: LitStr = nested.parse_args().unwrap();
                            Ident::new(&a.value(), a.span())
                        }
                        Meta::Path(_) => panic!("expected rename to have brackets with name of property"),
                        Meta::NameValue(_) => panic!("did not expect rename to have names and values")
                    }
                });

            if let Some(attr) = attr {
                quote! {
                    pub fn #attr(mut self, input: #field_type) -> Self {
                        self.#field_name = Some(input);
                        self
                    }
                }
            } else {
                quote! {
                    pub fn #field_name(mut self, input: #field_type) -> Self {
                        self.#field_name = Some(input);
                        self
                    }
                }
            }
        }).collect()
}


pub fn original_struct_setters(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream2> + '_ {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_as_string = field_name
            .as_ref().unwrap().to_string();

        quote! {
            #field_name: self.#field_name
                .expect(concat!("field not set: ", #field_name_as_string))
        }
    })
}

pub fn builder_init_values(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream2> + '_ {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: None }
    })
}

pub fn builder_field_definitions(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream2> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! { #field_name: Option<#field_type> }
    })
}

fn get_name_and_type<'a>(f: &'a Field) -> (&'a Option<Ident>, &'a Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
}

fn extract_attribute_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a syn::Attribute> {
    f.attrs.iter().find(|&attr| attr.path().is_ident(name))
}

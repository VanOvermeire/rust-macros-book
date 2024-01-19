use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, Field, Ident, Lit, Meta, MetaNameValue, Type};
use syn::punctuated::{Punctuated};
use syn::token::Comma;

pub fn builder_methods(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields.iter()
        .map(|f| {
            let (field_name, field_type) = get_name_and_type(f);

            extract_attribute_from_field(f, "rename")
                .map(|a| &a.meta)
                .map(|m| {
                    match m {
                        Meta::NameValue(MetaNameValue { value: Expr::Lit(ExprLit { lit: Lit::Str(literal_string), .. }), .. }) => {
                            Ident::new(&literal_string.value(), literal_string.span())
                        }
                       _ => panic!("expected key and value for rename attribute"),
                    }
                })
                .map(|attr| {
                    quote! {
                        pub fn #attr(mut self, input: #field_type) -> Self {
                            self.#field_name = Some(input);
                            self
                        }
                    }
                })
                .unwrap_or_else(|| {
                    quote! {
                        pub fn #field_name(mut self, input: #field_type) -> Self {
                            self.#field_name = Some(input);
                            self
                        }
                    }
                })
        }).collect()
}

pub fn original_struct_setters(fields: &Punctuated<Field, Comma>, use_defaults: bool) -> Vec<TokenStream> {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_as_string = field_name
            .as_ref().unwrap().to_string();

        let handle_type = if use_defaults {
            default_fallback()
        } else {
            panic_fallback(field_name_as_string)
        };

        quote! {
            #field_name: self.#field_name.#handle_type
        }
    })
        .collect()
}

// alternatives
// pub fn original_struct_setters<'a>(fields: &'a Punctuated<Field, Comma>, use_defaults: bool) -> Map<Iter<'a, Field>, Box<dyn Fn(&Field) -> TokenStream>>  {
//     fields.iter().map(Box::new(move |f| {
//         let (field_name, field_type) = get_name_and_type(&f);
//         let field_name_as_string = field_name.as_ref().unwrap().to_string();
//
//         let handle_type = if use_defaults {
//             default_fallback()
//         } else {
//             panic_fallback(field_name_as_string)
//         };
//
//         quote! {
//             #field_name: self.#field_name.#handle_type
//         }
//     }))
// }
//
// pub fn original_struct_setters(fields: &Punctuated<Field, Comma>, use_defaults: bool) -> impl Iterator<Item = TokenStream> + '_ {
//     fields.iter().map(move |f| {
//         let (field_name, field_type) = get_name_and_type(&f);
//         let field_name_as_string = field_name.as_ref().unwrap().to_string();
//
//         let handle_type = if use_defaults {
//             default_fallback()
//         } else {
//             panic_fallback(field_name_as_string)
//         };
//
//         quote! {
//             #field_name: self.#field_name.#handle_type
//         }
//     })
// }

fn panic_fallback(field_name_as_string: String) -> TokenStream {
    quote! {
        expect(concat!("Field not set: ", #field_name_as_string))
    }
}

fn default_fallback() -> TokenStream {
    quote! {
        unwrap_or_default()
    }
}

pub fn builder_init_values(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: None }
    })
}

pub fn builder_field_definitions(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
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

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, Field, Ident, Lit, Meta, MetaNameValue, Type};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
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

pub fn original_struct_setters(fields: &Punctuated<Field, Comma>, use_defaults: bool) -> Result<Vec<TokenStream>, syn::Error> {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        let field_name_as_string = field_name
            .as_ref().unwrap().to_string();

        let uppercase_attribute = extract_attribute_from_field(f, "uppercase");

        let to_add = if uppercase_attribute.is_some()
                && matches_type(field_type, "String") {
            quote! {
                .map(|v| v.to_uppercase())
            }
        } else if uppercase_attribute.is_some() {
            return Err(syn::Error::new(field_name.span(), "Can only use uppercase for String type"));
        } else {
            quote!()
        };

        let handle_type = if use_defaults {
            default_fallback()
        } else {
            panic_fallback(field_name_as_string)
        };

        Ok(quote! {
            #field_name: self.#field_name #to_add.#handle_type
        })
    })
        .collect()
}

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

pub fn builder_init_values(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item=TokenStream> + '_ {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: None }
    })
}

pub fn builder_field_definitions(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item=TokenStream> + '_ {
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

fn matches_type(ty: &Type, type_name: &str) -> bool {
    if let Type::Path(ref p) = ty {
        let first_match = p.path.segments[0].ident.to_string();
        return first_match == *type_name;
    }
    false
}

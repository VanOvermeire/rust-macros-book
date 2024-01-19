use proc_macro::TokenStream;

use quote::quote;
use syn::{Attribute, braced, Ident, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::token::Colon;

#[allow(dead_code)]
#[derive(Debug)]
struct StructWithComments {
    ident: Ident,
    field_name: Ident,
    field_type: Type,
    outer_attributes: Vec<Attribute>,
    inner_attributes: Vec<Attribute>,
}

impl Parse for StructWithComments {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let outer_attributes = input.call(Attribute::parse_outer).unwrap();
        let _: Token![struct] = input.parse().unwrap();
        let ident: Ident = input.parse().unwrap();

        let content;
        let _ = braced!(content in input);
        let inner_attributes = content.call(Attribute::parse_inner).unwrap();
        let field_name: Ident = content.parse().unwrap();
        let _: Colon = content.parse().unwrap();
        let field_type: Type = content.parse().unwrap();

        Ok(StructWithComments {
            ident,
            field_name,
            field_type,
            outer_attributes,
            inner_attributes,
        })
    }
}

#[proc_macro]
pub fn analyze(item: TokenStream) -> TokenStream {
    let _: StructWithComments = parse_macro_input!(item);
    quote!().into()
}

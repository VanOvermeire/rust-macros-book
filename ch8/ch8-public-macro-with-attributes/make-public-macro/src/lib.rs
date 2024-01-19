use proc_macro::TokenStream;

use quote::quote;
use syn::{DataStruct, DeriveInput, FieldsNamed, Ident, LitStr, MetaList, parse_macro_input, Token};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::meta::ParseNestedMeta;
use syn::token::Group;
use syn::PathArguments::Parenthesized;
use syn::punctuated::Punctuated;
use syn::PathSegment;
use syn::parse::Parser;

#[derive(Default)]
struct AlternativeExcludedFields {
    fields: Vec<String>,
}

impl AlternativeExcludedFields {
    fn matches_ident(&self, name: &Option<Ident>) -> bool {
        name.as_ref().map(|n| n.to_string())
            .map(|n| self.fields.iter().any(|f| *f == n))
            .unwrap_or_else(|| false)
    }
}

impl AlternativeExcludedFields {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<(), syn::Error> {
        if meta.path.is_ident(EXCLUDE_ATTRIBUTE_NAME) {
            meta.parse_nested_meta(|meta| {
                let ident = &meta.path.segments.first().unwrap().ident;
                self.fields.push(ident.to_string());
                Ok(())
            })
        } else {
            Err(meta.error("unsupported property"))
        }
    }
}

const EXCLUDE_ATTRIBUTE_NAME: &str = "exclude";

struct ExcludedFields {
    fields: Vec<String>,
}

impl ExcludedFields {
    fn matches_ident(&self, name: &Option<Ident>) -> bool {
        name.as_ref().map(|n| n.to_string())
            .map(|n| self.fields.iter().any(|f| *f == n))
            .unwrap_or_else(|| false)
    }
}

impl Parse for ExcludedFields {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        match input.parse::<MetaList>() {
            Ok(meta_list) => {
                if meta_list.path
                    .segments
                    .iter()
                    .find(|s| s.ident == EXCLUDE_ATTRIBUTE_NAME)
                    .is_some()
                {
                    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
                    let identifiers = parser.parse(meta_list.clone().tokens.into()).unwrap();
                    let fields = identifiers.iter()
                        .map(|v| v.to_string())
                        .collect();
                    Ok(ExcludedFields { fields })
                } else {
                    Ok(ExcludedFields { fields: vec![] })
                }
            }
            Err(_) => Ok(ExcludedFields { fields: vec![] })
        }
    }
}

#[proc_macro_attribute]
pub fn public(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    // let excluded_fields = parse_macro_input!(attr as ExcludedFields);

    let mut excluded_fields = AlternativeExcludedFields::default();
    let attr_parser = syn::meta::parser(|meta| excluded_fields.parse(meta));
    parse_macro_input!(attr with attr_parser);

    let name = ast.ident;

    let fields = match ast.data {
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
    };

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;

        if excluded_fields.matches_ident(name) {
            quote! { #vis #name: #ty }
        } else {
            quote! { pub #name: #ty }
        }
    });

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}

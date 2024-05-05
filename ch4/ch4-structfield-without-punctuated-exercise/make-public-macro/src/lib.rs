use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident, parse_macro_input, Type, Visibility};
use syn::Data::Struct;
use syn::DataStruct;
use syn::Fields::Named;
use syn::FieldsNamed;
use syn::parse::{Parse, ParseStream};
use syn::token::{Colon};

struct StructField {
    name: Ident,
    ty: Type,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

impl Parse for StructField {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis: Result<Visibility, _> = input.parse();
        let name: Ident = input.parse().unwrap();
        let _colon: Result<Colon, _> = input.parse();
        let ty: Type = input.parse().unwrap();

        Ok(StructField {
            name,
            ty,
        })
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
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

    let builder_fields = fields.iter()
        .map(|f| {
            syn::parse2::<StructField>(f.to_token_stream())
                .unwrap()
        });

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}
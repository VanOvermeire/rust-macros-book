use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, DataStruct, FieldsNamed, Data, Fields, Visibility};
use syn::__private::ToTokens;
use syn::Data::Struct;
use syn::Fields::Named;

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);

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
        quote! { pub #name: #ty }
    });
    let builder_fields_with_braces = quote!(
        {
            #(#builder_fields,)*
        }
    );

    ast.data = Data::Struct(DataStruct {
        struct_token: Default::default(),
        fields: Fields::Named(
            syn::parse2(builder_fields_with_braces).unwrap()
        ),
        semi_token: None,
    });
    ast.vis = Visibility::Public(Default::default());

    ast.to_token_stream().into()
}

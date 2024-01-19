use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(UpperCaseName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let uppercase_name = name.to_string().to_uppercase();

    let add_uppercase = quote! {
        impl #name {
            fn uppercase(&self) {
                println!("{}", #uppercase_name);
            }
        }
    };

    add_uppercase.into()
}

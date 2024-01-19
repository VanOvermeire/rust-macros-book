use quote::quote;
use proc_macro::TokenStream;

#[proc_macro_derive(Hello)]
pub fn hello(_item: TokenStream) -> TokenStream {
    let add_hello_world = quote! {};
    add_hello_world.into()
}

use proc_macro::TokenStream;
use builder_code::create_builder;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder(item: TokenStream) -> TokenStream {
    create_builder(item.into()).into()
}

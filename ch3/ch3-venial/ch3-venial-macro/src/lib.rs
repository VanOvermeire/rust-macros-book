use quote::quote;
use proc_macro::TokenStream;
use venial::{parse_declaration, Declaration, Struct, Enum};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let declaration = parse_declaration(item.into()).unwrap();

    let name = match declaration {
        Declaration::Struct(Struct { name, .. }) => name,
        Declaration::Enum(Enum { name, .. }) => name,
        _ => panic!("only implemented for struct and enum")
    };

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello world")
            }
        }
    };

    add_hello_world.into()
}
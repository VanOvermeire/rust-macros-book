use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType, Stmt};

fn signature_output_as_result(ast: &ItemFn) -> ReturnType {
    let output = match ast.sig.output {
        ReturnType::Default => {
            quote! {
                -> Result<(), String>
            }
        }
        ReturnType::Type(_, ref ty) => {
            quote! {
                -> Result<#ty, String>
            }
        }
    };
    syn::parse2(output).unwrap()
}

fn last_statement_as_result(last_statement: Option<Stmt>) -> Stmt {
    let last_unwrapped = last_statement.unwrap();
    let last_modified = quote! {
        Ok(#last_unwrapped)
    };
    Stmt::Expr(syn::parse2(last_modified).unwrap(), None)
}

#[proc_macro_attribute]
pub fn panic_to_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item).unwrap();

    ast.sig.output = signature_output_as_result(&ast);
    let last_statement = ast.block.stmts.pop();
    ast.block.stmts.push(last_statement_as_result(last_statement));

    ast.to_token_stream().into()
}


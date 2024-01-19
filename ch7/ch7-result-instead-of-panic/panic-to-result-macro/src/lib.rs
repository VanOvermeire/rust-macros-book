use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType, Stmt, Expr, StmtMacro};
use syn::token::Semi;

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

fn handle_expression(expression: Expr, token: Option<Semi>) -> Stmt {
    match expression {
        Expr::If(mut ex_if) => {
            let new_statements: Vec<Stmt> = ex_if.then_branch.stmts.into_iter()
                .map(|s| match s {
                    Stmt::Macro(ref expr_macro) =>
                            extract_panic_content(expr_macro)
                                .map(|t| quote! {
                                        return Err(#t.to_string());
                                    })
                                .map(syn::parse2)
                                .map(Result::unwrap)
                                .unwrap_or(s),
                    _ => s
                })
                .collect();
            ex_if.then_branch.stmts = new_statements;
            Stmt::Expr(Expr::If(ex_if), token)
        },
        _ => Stmt::Expr(expression, token)
    }
}

fn extract_panic_content(expr_macro: &StmtMacro) -> Option<proc_macro2::TokenStream> {
    let does_panic = expr_macro.mac.path.segments.iter()
        .any(|v| v.ident.to_string().eq("panic"));

    if does_panic {
        Some(expr_macro.mac.tokens.clone())
    } else {
        None
    }
}

#[proc_macro_attribute]
pub fn panic_to_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item).unwrap();

    let new_statements: Vec<Stmt> = ast.block.stmts
        .into_iter()
        .map(|s| match s {
            Stmt::Expr(e, t) => handle_expression(e, t),
            _ => s,
        })
        .collect();
    ast.block.stmts = new_statements;

    ast.sig.output = signature_output_as_result(&ast);
    let last_statement = ast.block.stmts.pop();
    ast.block.stmts.push(last_statement_as_result(last_statement));

    ast.to_token_stream().into()
}


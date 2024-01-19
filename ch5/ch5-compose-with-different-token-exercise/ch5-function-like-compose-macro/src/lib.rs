use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Not;

struct ComposeInput {
    expressions: Punctuated::<Ident, Not>,
}

impl Parse for ComposeInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(
            ComposeInput {
                expressions: Punctuated::<Ident, Token!(!)>
                    ::parse_terminated(input).unwrap(),
            }
        )
    }
}

impl ToTokens for ComposeInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut total = None;
        let mut as_idents: Vec<&Ident> = self.expressions
            .iter()
            .collect();
        let last_ident = as_idents
            .pop()
            .unwrap();

        as_idents.iter()
            .rev()
            .for_each(|i| {
                if let Some(current_total) = &total {
                    total = Some(quote!(
                        compose_two(#i, #current_total)
                    ));
                } else {
                    total = Some(quote!(
                        compose_two(#i, #last_ident)
                    ));
                }
            });
        total.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn compose(item: TokenStream) -> TokenStream {
    let ci: ComposeInput = parse_macro_input!(item);

    quote!(
        {
            fn compose_two<FIRST, SECOND, THIRD, F, G>(first: F, second: G)
            -> impl Fn(FIRST) -> THIRD
            where
                F: Fn(FIRST) -> SECOND,
                G: Fn(SECOND) -> THIRD,
            {
                move |x| second(first(x))
            }
            #ci
        }
    ).into()
}

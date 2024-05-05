use proc_macro::TokenStream;

use quote::quote;
use syn::{Ident, LitInt, parenthesized, Token};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

pub(crate) mod kw {
    syn::custom_keyword!(bucket);
    syn::custom_keyword!(lambda);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Bucket {
    pub name: String,
    pub event: bool,
}

impl Parse for Bucket {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let bucket_token = input.parse::<kw::bucket>().expect("we just checked for this token");
        let bucket_name = input.parse()
            .map(|v: Ident| v.to_string())
            .map_err(|_| syn::Error::new(
                bucket_token.span(),
                "bucket needs a name")
            )?;

        let event_needed = if !input.peek(kw::lambda) && input.peek(Token!(=>)) {
            let _ = input.parse::<Token!(=>)>().unwrap();
            true
        } else {
            false
        };

        Ok(Bucket {
            name: bucket_name,
            event: event_needed,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Lambda {
    pub name: String,
    pub memory: Option<u16>,
    pub time: Option<u16>,
}

#[derive(Debug)]
struct KeyValue {
    pub key: String,
    pub value: String,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let key = input.parse()
            .map(|v: Ident| v.to_string())
            .map_err(|_| syn::Error::new(
                input.span(),
                "should have property keys within parentheses"
            ))?;
        let _: Token!(=) = input.parse()
            .map_err(|_| syn::Error::new(
                input.span(),
                "prop name and value should be separated by an equals sign",
            ))?;

        let value = if key == "name" {
            input.parse()
                .map(|v: Ident| v.to_string())
                .map_err(|_| syn::Error::new(
                    input.span(),
                    "name property needs a value",
                ))
        } else if key == "mem" || key == "time" {
            input.parse()
                .map(|v: LitInt| v.to_string())
                .map_err(|_| {
                    syn::Error::new(
                        input.span(),
                        "memory and time needs a positive value")
                })
        } else {
            Err(syn::Error::new(
                input.span(),
                format!("unknown property for lambda: {}", key
                )))
        }?;

        Ok(KeyValue {
            key,
            value,
        })
    }
}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _ = input.parse::<kw::lambda>().expect("we just checked for this token");
        let mut lambda_name = None;
        let mut lambda_memory = None;
        let mut lambda_timeout = None;

        let content;
        parenthesized!(content in input);

        let kvs = Punctuated::<KeyValue, Token!(,)>::parse_terminated(&content)?;
        kvs.into_iter().for_each(|kv| {
            if kv.key == "name" {
                lambda_name = Some(kv.value);
            } else if kv.key == "mem" {
                lambda_memory = Some(kv.value.parse().unwrap()); // should actually check like before
            } else if kv.key == "time" {
                lambda_timeout = Some(kv.value.parse().unwrap());
            }
        });

        Ok(Lambda {
            name: lambda_name.ok_or(syn::Error::new(
                input.span(),
                "lambda needs a name",
            ))?,
            memory: lambda_memory,
            time: lambda_timeout,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct IacInput {
    bucket: Option<Bucket>,
    lambda: Option<Lambda>,
}

impl Parse for IacInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut bucket: Option<Bucket> = None;
        let mut lambda = None;

        loop {
            if input.peek(kw::bucket) {
                bucket = Some(input.parse()?);
            } else if input.peek(kw::lambda) {
                lambda = Some(input.parse()?);
            } else if !input.is_empty() {
                return Err(syn::Error::new(
                    input.lookahead1().error().span(),
                    "only 'bucket' and 'lambda' resources are supported",
                ));
            } else {
                break; // no input left - stop
            }
        }

        if bucket.as_ref().map(|v| v.event).unwrap_or(false) && lambda.is_none() {
            return Err(syn::Error::new(
                input.span(),
                "a lambda is required for an event ('=>')")
            );
        }

        Ok(
            IacInput {
                bucket,
                lambda,
            }
        )
    }
}

#[proc_macro]
pub fn iac(item: TokenStream) -> TokenStream {
    let ii: IacInput = parse_macro_input!(item);
    eprintln!("{:?}", ii);
    quote!().into()
}

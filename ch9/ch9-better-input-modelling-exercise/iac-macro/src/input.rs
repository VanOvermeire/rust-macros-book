use syn::{Ident, LitInt, Token};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

pub(crate) mod kw {
    syn::custom_keyword!(bucket);
    syn::custom_keyword!(lambda);
    syn::custom_keyword!(mem);
    syn::custom_keyword!(time);
}

#[derive(Debug)]
pub struct Bucket {
    pub name: String,
    has_event: bool,
}

impl Parse for Bucket {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let bucket_token = input.parse::<kw::bucket>()
            .expect("we just checked for this token");
        let bucket_name = input.parse()
            .map(|v: Ident| v.to_string())
            .map_err(|_| syn::Error::new(
                bucket_token.span(),
                "bucket needs a name")
            )?;

        let event_needed = if !input.peek(kw::lambda) && input.peek(Token!(=>)) {
            // by peeking, we know the token is present. and we want to get rid of it
            let _ = input.parse::<Token!(=>)>().unwrap();
            true
        } else {
            false
        };

        Ok(Bucket {
            name: bucket_name,
            has_event: event_needed,
        })
    }
}

#[derive(Debug)]
pub struct Lambda {
    pub name: String,
    pub memory: Option<u16>,
    pub time: Option<u16>,
}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lambda_token = input.parse::<kw::lambda>()
            .expect("we just checked for this token");
        let lambda_name = input.parse()
            .map(|v: Ident| v.to_string())
            .map_err(|_| {
                syn::Error::new(lambda_token.span, "lambda needs a name")
            })?;
        let mut lambda_memory = None;
        let mut lambda_timeout = None;

        while !input.is_empty() && !input.peek(kw::bucket) {
            if input.peek(kw::mem) {
                let _ = input.parse::<kw::mem>().expect("we just checked for this token");
                lambda_memory = Some(
                    input.parse()
                        .map(|v: LitInt| v.to_string()
                            .parse()
                            .map_err(|_| {
                                syn::Error::new(
                                    v.span(),
                                    "memory needs a positive value <= 10240",
                                )
                            })
                        )??
                );
            } else if input.peek(kw::time) {
                let _ = input.parse::<kw::time>()
                    .expect("we just checked for this token");
                lambda_timeout = Some(
                    input.parse()
                        .map(|v: LitInt| v.to_string()
                            .parse()
                            .map_err(|_| {
                                syn::Error::new(
                                    v.span(),
                                    "timeout needs a positive value <= 900",
                                )
                            })
                        )??
                );
            } else {
                Err(syn::Error::new(input.span(), "unknown property passed to lambda"))?
            }
        }

        Ok(Lambda {
            name: lambda_name,
            memory: lambda_memory,
            time: lambda_timeout,
        })
    }
}

#[derive(Debug)]
pub enum IacInput {
    Normal(Option<Bucket>, Option<Lambda>),
    EventBucket(Bucket, Lambda),
}

impl IacInput {
    pub fn has_resources(&self) -> bool {
        if is_ide_completion() {
            return false;
        }

        match self {
            IacInput::EventBucket(_, _) => true,
            IacInput::Normal(None, None) => false,
            _ => true,
        }
    }
}

fn is_ide_completion() -> bool {
    match std::env::var_os("RUST_IDE_PROC_MACRO_COMPLETION_DUMMY_IDENTIFIER") {
        None => false,
        Some(dummy_identifier) => !dummy_identifier.is_empty(),
    }
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
                    "only 'bucket' and 'lambda' resources are supported")
                );
            } else {
                break; // no input left, stop
            }
        }

        if bucket.as_ref().map(|v| v.has_event).unwrap_or(false) {
            return if lambda.is_none() {
                Err(syn::Error::new(
                    input.span(),
                    "a lambda is required for an event ('=>')")
                )
            } else {
                Ok(IacInput::EventBucket(
                    bucket.expect("only here when bucket exists"),
                    lambda.expect("just checked that this exists"),
                ))
            }
        }
        Ok(IacInput::Normal(bucket, lambda))
    }
}

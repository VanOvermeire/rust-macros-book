use syn::{LitStr, Token};
use syn::parse::{Parse, ParseStream};

pub(crate) mod kw {
    syn::custom_keyword!(path);
}

#[derive(Debug)]
pub struct ConfigInput {
    pub path: Option<String>,
}

impl Parse for ConfigInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(ConfigInput {
                path: None,
            });
        }

        if !input.peek(kw::path) {
            return Err(
                syn::Error::new(
                    input.span(),
                    "config macro only allows for 'path' input",
                )
            );
        }

        let _: kw::path = input.parse()
            .expect("checked that this exists");
        let _: Token!(=) = input.parse()
            .map_err(|_| syn::Error::new(
                input.span(),
                "expected equals sign after path"
            ))?;
        let value: LitStr = input.parse()
            .map_err(|_| syn::Error::new(
                input.span(),
                "expected value after the equals sign"
            ))?;

        Ok(ConfigInput {
            path: Some(value.value()),
        })
    }
}

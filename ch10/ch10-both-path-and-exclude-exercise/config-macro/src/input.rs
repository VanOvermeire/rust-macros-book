use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

pub(crate) mod kw {
    syn::custom_keyword!(path);
    syn::custom_keyword!(exclude);
}

#[derive(Debug)]
pub struct ConfigInput {
    pub path: Option<String>,
    pub exclude_from: bool,
}

impl Parse for ConfigInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut path = None;
        let mut exclude_from = None;

        while !input.is_empty() {
            if input.peek(kw::path) {
                let _: kw::path = input.parse()
                    .expect("checked that this exists");
                let _: Token!(=) = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected equals sign after path"))?;
                let value: LitStr = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected value after the equals sign"))?;

                path = Some(value.value());
            } else if input.peek(kw::exclude) {
                let _: kw::exclude = input.parse()
                    .expect("checked that this exists");
                let _: Token!(=) = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected equals sign after path"))?;
                let value: LitStr = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected value after the equals sign"))?;

                exclude_from = Some(value.value() == "from");
            } else {
                return Err(
                    syn::Error::new(
                        input.span(),
                        "config macro only allows for 'path' input",
                    )
                );
            }
        }

        Ok(ConfigInput {
            path,
            exclude_from: exclude_from.unwrap_or(false),
        })
    }
}

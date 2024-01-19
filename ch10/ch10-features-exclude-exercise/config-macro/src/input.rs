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

// (only allows either path or exclude)
impl Parse for ConfigInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(ConfigInput {
                path: None,
                exclude_from: false,
            })
        } else if input.peek(kw::path) {
            let _: kw::path = input.parse()
                .expect("checked that this exists");
            let _: Token!(=) = input.parse()
                .map_err(|_| syn::Error::new(input.span(), "expected equals sign after path"))?;
            let value: LitStr = input.parse()
                .map_err(|_| syn::Error::new(input.span(), "expected value after the equals sign"))?;

            Ok(ConfigInput {
                path: Some(value.value()),
                exclude_from: false,
            })
        } else {
            Err(
                syn::Error::new(
                    input.span(),
                    "config macro only allows for 'path' input",
                )
            )
        }
    }
}

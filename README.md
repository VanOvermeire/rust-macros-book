# Write Powerful Rust Macros

## Overview

This is the code for Manning's _Write Powerful Rust Macros_ [book](http://mng.bz/e1lv). 

The code is organized by chapter. Exercise solutions are _not_ located in a separate appendix directory, instead they sit next to the example code for the chapter they appeared in, and always end in `-exercise`.
Every project besides the exercise solutions represents either a complete example, or (since many chapters are based around a single example) show stages in the development of that example.
Each one contains a README file that gives some idea what part of the chapter it is related to.

## Util

The `util` directory contains a script, `create_setup.sh` that allows you to generate procedural macro setups like the ones used in this book, with either 1, 2, or 3 subdirectories.
All three options are explored in the book.

Usage:

```bash
./create_setup.sh name 2
```

## Errata

Thanks again to the readers who reported these.

**Section 1.2 (page 4)**

C++, not C, has templates

**Section 1.3.2 (page 7)**

`#[derive(Clone)}` should be `#[derive(Clone)]`

**Section 2.1.4 (page 15)**

_Nonemtpy_ in the title should be _nonempty_

**Section 2.1.4 (page 18)**

`$[($x:expr),+] => (:` should not end with `:`

**Section 2.4 (page 39)**

_trailing comments_ should be _trailing commas_

**Section 5.1 (page 82, 84, and 87)**

In this chapter `quote::__private::TokenStream` should be `proc_macro2::TokenStream`.

**Section 5.1 (page 84)**

For the 'ugly return type' a reference to what is currently pages 264–265 would be useful.

_Now focus on the code inside map._ would be clearer with a reference to the listing: _Now focus on the code inside map in listing 5.4._

For clarity, replace _So we need an identifier._ with the following:
_The simplest way to solve the issue is to use `TokenStream::from_str`. This method will try to turn your string into a `TokenStream` that we can use as an identifier. The other approach is to construct an identifier ourselves._

**Section 6.4.3 (page 114)**

Minor: `pub` function could be `pub(crate)`.

**Section 6.4.5 (page 120)**

`assert_eq!(gleipnir.other_necessities.len(), 3)` should preferably be `assert_eq!(gleipnir.other_necessities.len(), 3);`

**Section 7.5 (page 138)**

`use quote::{ToTokens};` should be `use quote::ToTokens;`

**Section 7.7 (page 143)**

`ast.to_token_stream()` should be `ast.to_token_stream().into()`. The `into()` seems to have disappeared in the proofs, it was present in the code.

**Section 8.1 (page 164)**

The tests are added to `main.rs` of `builder-usage` (not really a bug, but it's unclear where the given tests should be placed).

**Section 10.1.2 (page 237)**

The `Ok` and `?` are redundant. So this:

```
Ok(serde_yaml::from_reader(file)
        .map_err(|e| {
            syn::Error::new(Span::call_site(), e.to_string())
        })?)
```

Can be changed to

```
serde_yaml::from_reader(file)
        .map_err(|e| {
            syn::Error::new(Span::call_site(), e.to_string())
        })
```

**Appendix (page 260)**

_trailing comments_ should be _trailing commas_

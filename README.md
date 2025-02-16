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

**Section 6.4.3 (page 114)**

Minor: pub functions could be pub(crate).

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

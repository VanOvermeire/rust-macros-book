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

**Section 1.3.2 (page 7)**

`#[derive(Clone)}` should be `#[derive(Clone)]`

**Section 2.1.4 (page 15)**

"Nonemtpy" in the title should be "nonempty"

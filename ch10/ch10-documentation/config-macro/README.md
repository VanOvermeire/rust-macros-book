# Config Macro

## Overview

This crate contains macros that allow you to transform yaml config into a struct that you can use in your application.

## Usage

Call either the function-like `config!` macro or annotate a (preferably empty) struct with the attribute macro `#[config_struct]`.
By default, the macro looks for configuration under `configuration/config.yaml`.
This can be overwritten by using the 'path' attribute: `config!(path = "a/path/to.yaml")` or `#[config_struct(path = "a/path/to.yaml")]`.

## Usage example

```rust
use config_macro::config;

config!(path  = "./configuration/config.yaml");

// we can now call new and access the hashmap of values
let c = Config::new();
```

## Features

The annotation macro is hidden behind the 'struct' feature.

## Caveats

Currently only works with YAML files, and does not support nesting.
So this works:

```yaml
user: "admin"
```

But this won't:

```yaml
database:
  user: "admin"
```

Furthermore, we will read all properties in the config as `String`s.

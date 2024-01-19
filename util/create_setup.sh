#!/bin/bash

# Simple helper that allows you to create a derive macro in the 'setup styles' used in this book

# This means that this helper will create a project with 1-3 subdirectories *in the current directory*
# (In the early chapters of the book, we often start with one directory, later we switch to using two or three)

# In the case of a one subdirectory setup, the macro directory will be called '{chosen-name}-macro'
# In the case of a two subdirectory setup, the directories will be called '{chosen-name}-macro' and '{chosen-name}-usage'
# In the case of a three subdirectory setup, the directories will be called '{chosen-name}-code', '{chosen-name}-macro' and '{chosen-name}-usage'

# NOTES:
# - assumes the commands `cargo init` and `cargo add` will work
# - adds the latest version of quote and syn, which will probably work, but is not guaranteed to -> check the versions used in the book / code if you run into issues

set -euo pipefail

if [ $# -ne 2 ]; then
  echo "Usage create_setup.sh <PROJECT_NAME> <NUMBER_OF_DIRS (1|2|3)>>"
  exit 1
fi

PROJECT_NAME=$1
NUMBER_OF_DIRS=$2

if [[ $NUMBER_OF_DIRS != 1 && $NUMBER_OF_DIRS != 2 && $NUMBER_OF_DIRS != 3 ]]; then
  echo "NUMBER_OF_DIRS is either 1, 2 or 3"
  exit 1
fi

echo "Creating project with $PROJECT_NAME and $NUMBER_OF_DIRS directories"

macro_code_dir="$PROJECT_NAME-code"
macro_dir="$PROJECT_NAME-macro"
project_dir="$PROJECT_NAME-usage"

function set_as_proc_macro() {
    {
      echo ""
      echo "[lib]"
      echo "proc-macro = true"
    } >> Cargo.toml
}

function example_macro() {
    {
      echo "use proc_macro::TokenStream;"
      echo "use quote::quote;"
      echo ""
      echo "#[proc_macro_derive(Example)]"
      echo "pub fn example(_: TokenStream) -> TokenStream {"
      echo "    quote!().into()"
      echo "}"
    } > src/lib.rs
}

function create_with_one_dir() {
    mkdir "$macro_dir"
    cd "$macro_dir" && cargo init --lib && cargo add syn && cargo add quote
    set_as_proc_macro
    example_macro

    cd ..

    cargo init

    {
      echo "$macro_dir = { path = \"./$macro_dir\" }"
    } >> Cargo.toml
}

function create_with_two_dirs() {
    mkdir "$macro_dir"
    cd "$macro_dir" && cargo init --lib && cargo add syn && cargo add quote
    set_as_proc_macro
    example_macro

    cd ..

    mkdir "$project_dir"
    cd "$project_dir" && cargo init
    {
      echo "$macro_dir = { path = \"../$macro_dir\" }"
    } >> Cargo.toml
}

function create_with_three_dirs() {
    mkdir "$macro_code_dir"
    cd "$macro_code_dir" && cargo init --lib && cargo add syn && cargo add quote && cargo add proc-macro2
    {
      echo "use proc_macro2::TokenStream;"
      echo "use quote::quote;"
      echo ""
      echo "pub fn example_code(_: TokenStream) -> TokenStream {"
      echo "    quote!().into()"
      echo "}"
    } > src/lib.rs

    cd ..

    mkdir "$macro_dir"
    cd "$macro_dir" && cargo init --lib
    {
      echo "$macro_code_dir = { path = \"../$macro_code_dir\" }"
    } >> Cargo.toml

    set_as_proc_macro

    underscores_macro_code_dir=$(echo "$macro_code_dir" | tr - _)

    {
      echo "use proc_macro::TokenStream;"
      echo "use $underscores_macro_code_dir::example_code;"
      echo ""
      echo "#[proc_macro_derive(Example)]"
      echo "pub fn example(input: TokenStream) -> TokenStream {"
      echo "    example_code(input.into()).into()"
      echo "}"
    } > src/lib.rs

    cd ..

    mkdir "$project_dir"
    cd "$project_dir" && cargo init
    {
      echo "$macro_dir = { path = \"../$macro_dir\" }"
    } >> Cargo.toml
}

if [[ $NUMBER_OF_DIRS == 1 ]]; then
  create_with_one_dir
elif [[ $NUMBER_OF_DIRS == 2 ]]; then
  create_with_two_dirs
else
  create_with_three_dirs
fi

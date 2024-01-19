use private_macro_with_private_fields::private;

private!(
    struct Example {
        pub string_value: String,
        pub number_value: i32,
    }
);

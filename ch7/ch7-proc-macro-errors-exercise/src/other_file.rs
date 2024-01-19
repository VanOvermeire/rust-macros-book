use proc_macro_errors_exercise::private;

private!(
    struct Example {
        pub string_value: String,
        pub number_value: i32,
    }
);

// will fail
// private!(
//     enum Example {
//         First
//     }
// );

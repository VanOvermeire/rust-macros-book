#![feature(trace_macros)]
#![feature(log_syntax)]

use crate::greeting::base_greeting_fn;
#[macro_use]
mod greeting;

fn main() {
    trace_macros!(true);
    let _greet = greeting!("Sam", "Heya");
    let _greet_with_default = greeting!("Sam");
    let _greet_with_default_test = greeting!(test "Sam");
    trace_macros!(false);
}

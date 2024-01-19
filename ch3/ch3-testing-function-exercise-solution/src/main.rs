#[macro_use]
extern crate hello_world_testing_function_macro;

#[derive(Hello)]
struct Example;

fn main() {
    Example::testing_testing();
}
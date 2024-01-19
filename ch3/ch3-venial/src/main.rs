#[macro_use]
extern crate venial_macro;

#[derive(Hello)]
struct Example;

fn main() {
    let e = Example {};
    e.hello_world();
}
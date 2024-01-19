#[macro_use]
extern crate hello_world_implemented_no_syn_or_quote_macro;

#[derive(Hello)]
struct Example;

fn main() {
    let e = Example {};
    e.hello_world();
}

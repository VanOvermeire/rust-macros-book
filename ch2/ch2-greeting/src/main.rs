use crate::greeting::base_greeting_fn;
// use crate::greeting::greeting;
#[macro_use]
mod greeting;

fn main() {
    let greet = greeting!("Sam", "Heya");
    println!("{}", greet);
    let greet_with_default = greeting!("Sam");
    println!("{}", greet_with_default);
}

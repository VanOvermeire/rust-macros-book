use crate::example::{AlsoWorksExample, Example};

mod example;

fn main() {
    let _ = AlsoWorksExample {
        first: "".to_string(),
        second: 0,
    };
    let e = Example::new();
    println!("{}", e.first);
    println!("{}", e.second);
    // println!("{}", e.third); // won't work
}

use make_public_macro::public;

#[public]
struct Example {
    first: String,
    pub second: u32,
}

fn main() {
    let _e = Example {
        first: "first".to_string(),
        second: 5,
    };
}

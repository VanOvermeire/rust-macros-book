use make_public_macro::public;

#[public(exclude(fourth, third))]
struct Example {
    first: String,
    pub second: u32,
    third: bool,
    fourth: String,
}

#[public]
struct AlsoWorksExample {
    first: String,
    pub second: u32,
}

impl Example {
    pub fn new() -> Self {
        Example {
            first: "first".to_string(),
            second: 5,
            third: false,
            fourth: "fourth".to_string(),
        }
    }
}
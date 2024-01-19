use make_public_structfield_without_punctuated_exercise_macro::public;

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

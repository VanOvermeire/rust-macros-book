use hello_world_fill_in_blanks_macro::UpperCaseName;

#[derive(UpperCaseName)]
struct Example;

fn main() {
    let e = Example {};
    e.uppercase();
}
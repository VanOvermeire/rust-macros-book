use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[panic_to_result]
fn create_person_with_empty_panic(name: String, age: u32) -> Person {
    if age > 30 {
        panic!();
    }
    Person {
        name,
        age,
    }
}

fn main() {}

use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[panic_to_result]
fn create_person_with_result(name: String, age: u32) -> Result<Person, String> {
    if age > 30 {
        panic!("I hope I die before I get old");
    }
    Ok(Person {
        name,
        age,
    })
}

fn main() {}

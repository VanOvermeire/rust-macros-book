use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[panic_to_result]
fn create_person_two_issues(name: String, age: u32) -> Result<String, Person> {
    if age > 30 {
        panic!();
    }
    Ok(Person {
        name,
        age,
    })
}

fn main() {}

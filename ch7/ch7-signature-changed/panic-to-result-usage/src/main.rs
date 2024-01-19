use panic_to_result_macro::panic_to_result;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[panic_to_result]
fn create_person(name: String, age: u32) -> Person {
    if age > 30 {
        panic!("I hope I die before I get old");
    }
    Person {
        name,
        age,
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let actual = create_person("Sam".to_string(), 22).unwrap();

        assert_eq!(actual.name, "Sam".to_string());
        assert_eq!(actual.age, 22);
    }

    #[test]
    #[should_panic]
    fn should_panic_on_invalid_age() {
        let _ = create_person("S".to_string(), 32);
    }
}
struct FirstNameExample {
    value: String,
}

impl FirstNameExample {
    pub fn new(name: &str) -> Result<FirstNameExample, String> {
        if name.len() < 2 {
            Err("Name should be at least two characters".to_string())
        } else {
            Ok(FirstNameExample {
                value: name.to_string(),
            })
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

struct FirstName {
    value: String,
}

struct LastName {
    value: String,
}

struct Age {
    value: i32,
}

struct Pay {
    value: i32,
}

macro_rules! generate_get_value_string {
    ($struct_type:ident) => {
        generate_get_value_string!($struct_type,String);
    };
    ($struct_type:ident,$return_type:ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    }
}

generate_get_value_string!(FirstName);
generate_get_value_string!(LastName);
generate_get_value_string!(Age,i32);
generate_get_value_string!(Pay,i32);

fn calculate_raise(first_name: String,
                   _last_name: String,
                   _age: i32,
                   current_pay: i32) -> i32 {
    if first_name == "Sam" {
        current_pay + 1000
    } else {
        current_pay
    }
}

fn main() {
    let first_raise = calculate_raise(
        "Smith".to_string(),
        "Sam".to_string(),
        20,
        1000
    );
    println!("{}", first_raise);

    let second_raise = calculate_raise(
        "Sam".to_string(),
        "Smith".to_string(),
        1000,
        20
    );
    println!("{}", second_raise);
}

#[cfg(test)]
mod tests {
    use crate::FirstNameExample;

    #[test]
    fn should_create_first_name_example() {
        let actual = FirstNameExample::new("Sam").unwrap();

        assert_eq!(actual.get_value(), "Sam");
    }

    #[test]
    fn should_fail_to_create_first_name_example_that_is_too_short() {
        let actual = FirstNameExample::new("S");

        assert!(actual.is_err());
    }
}

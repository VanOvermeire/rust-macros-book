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

macro_rules! generate_from {
    ($struct_type:ident) => {
        generate_from!($struct_type,String);
    };
    ($struct_type:ident,$return_type:ty) => {
        impl From<$struct_type> for $return_type {
            fn from(f: $struct_type) -> Self {
                f.value
            }
        }
    }
}

macro_rules! generate_from {
    ($struct_type:ident) => {
        generate_from!($struct_type,String);
    };
    ($struct_type:ident,$return_type:ty) => {
        impl From<$struct_type> for $return_type {
            fn from(f: $struct_type) -> Self {
                f.value
            }
        }
    }
}

macro_rules! generate_newtypes_methods {
    ($struct_type:ident) => {
        generate_get_value_string!($struct_type,String);
        generate_from!($struct_type,String);
    };
    ($struct_type:ident,$return_type:ty) => {
        generate_get_value_string!($struct_type,$return_type);
        generate_from!($struct_type,$return_type);
    }
}

// => now we can generate everything we need with one call
generate_newtypes_methods!(FirstName);
generate_newtypes_methods!(LastName);
generate_newtypes_methods!(Age,i32);
generate_newtypes_methods!(Pay,i32);

fn main() {}

#[cfg(test)]
mod tests {
    use crate::FirstName;

    #[test]
    fn should_have_working_into_impl() {
        let first_name = FirstName {
            value: "Hello".to_string()
        };

        let actual: String = first_name.into();

        assert_eq!(actual, "Hello".to_string());
    }

    #[test]
    fn should_have_working_from_impl() {
        let first_name = FirstName {
            value: "Hello".to_string()
        };

        let actual: String = String::from(FirstName { value: "Hello".to_string() });

        assert_eq!(actual, "Hello".to_string());
    }
}

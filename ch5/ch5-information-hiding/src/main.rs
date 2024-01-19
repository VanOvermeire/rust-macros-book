use private_macro::private;

private!(
    struct Example {
        string_value: String,
        number_value: i32,
    }
);

fn main() {
    let e = Example {
        string_value: "value".to_string(),
        number_value: 2,
    };

    e.get_string_value();
    e.get_number_value();
}

#[cfg(test)]
mod tests {
    use crate::Example;

    #[test]
    fn generates_necessary_methods() {
        let e = Example {
            string_value: "value".to_string(),
            number_value: 2,
        };

        assert_eq!(e.get_string_value(), &"value");
        assert_eq!(e.get_number_value(), &2);
    }
}
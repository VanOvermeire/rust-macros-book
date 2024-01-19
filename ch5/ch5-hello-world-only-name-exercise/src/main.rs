use hello_world_only_name_exercise_macro::hello;

struct Example {
    another_value: String
}

hello!(Example);

fn main() {
    let e = Example {
        another_value: "does not disappear".to_string(),
    };
    e.hello_world();
}

#[cfg(test)]
mod tests {
    use crate::Example;

    #[test]
    fn value_does_not_disappear() {
        let e = Example {
            another_value: "does not disappear".to_string(),
        };

        assert_eq!(e.another_value, "does not disappear".to_string());
    }

    #[test]
    fn hello_world_method_available() {
        let e = Example {
            another_value: "does not disappear".to_string(),
        };

        e.hello_world();
    }
}
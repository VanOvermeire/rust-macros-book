use function_like_compose_macro::compose;

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

fn main() {
    let compose = compose!(add_one . add_one . stringify);
    println!("{:?}", compose(5));
}

#[cfg(test)]
mod tests {
    use super::*;
    use function_like_compose_macro::compose;

    #[test]
    fn basic_test() {
        let compose = compose!(add_one . add_one . stringify);

        let actual = compose(5);

        assert_eq!(actual, "7");
    }

    #[test]
    fn two_elements_test() {
        let compose = compose!(add_one . stringify);

        let actual = compose(5);

        assert_eq!(actual, "6");
    }

    #[test]
    fn four_elements_test() {
        let compose = compose!(add_one . add_one . add_one . add_one);

        let actual = compose(12);

        assert_eq!(actual, 16);
    }
}

use function_like_compose_with_different_token_exercise_macro::compose;

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

fn main() {
    let compose = compose!(add_one ! add_one ! stringify);
    println!("{:?}", compose(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let compose = compose!(add_one ! add_one ! stringify);

        let actual = compose(5);

        assert_eq!(actual, "7");
    }
}

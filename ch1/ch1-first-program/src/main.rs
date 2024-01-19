use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    given_name: String,
    last_name: String,
}

fn full_name(given: &str, last: &str) -> String {
    format!("{} {}", given, last)
}

fn main() {
    let r = Request {
        given_name: "Sam".to_string(),
        last_name: "Hall".to_string()
    };
    dbg!(full_name(&r.given_name, &r.last_name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let actual: Request = serde_json::from_str("{ \"given_name\": \"Test\", \"last_name\": \"McTest\" }")
            .expect("deserialize to work");

        assert_eq!(actual.given_name, "Test".to_string());
        assert_eq!(actual.last_name, "McTest".to_string());
    }
}
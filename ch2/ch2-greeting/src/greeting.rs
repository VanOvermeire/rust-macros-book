pub fn base_greeting_fn(name: &str, greeting: &str) -> String {
    format!("{}, {}!", greeting, name)
}

macro_rules! greeting {
    ($name:literal) => {
        base_greeting_fn($name,"Hello")
    };
    ($name:literal,$greeting:literal) => {
        base_greeting_fn($name,$greeting)
    }
}

// pub(super) use greeting;

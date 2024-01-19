pub fn base_greeting_fn(name: &str, greeting: &str) -> String {
    format!("{}, {}!", greeting, name)
}

macro_rules! greeting {
    ($name:literal) => {
        base_greeting_fn($name,"Hello")
    };
    ($name:literal,$greeting:literal) => {
        base_greeting_fn($name,$greeting)
    };
    (test $name:literal) => {{
        log_syntax!("The name passed to test is ", $name);
        println!("Returning default greeting");
        base_greeting_fn($name,"Hello")
   }}
}

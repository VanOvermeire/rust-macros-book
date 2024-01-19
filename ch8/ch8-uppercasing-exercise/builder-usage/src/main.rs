fn main() {}

#[cfg(test)]
mod tests {
    use builder_macro::Builder;

    #[test]
    fn should_uppercase_the_attribute() {
        #[derive(Builder)]
        struct Gleipnir {
            #[uppercase]
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("upper".to_string())
            .build();

        assert_eq!(gleipnir.roots_of, "UPPER".to_string());
    }
}

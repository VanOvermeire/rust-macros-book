fn main() {}

#[cfg(test)]
mod tests {
    use builder_macro::Builder;

    #[test]
    fn should_use_defaults_when_attribute_is_present() {
        #[derive(Builder)]
        #[builder_defaults]
        struct Gleipnir {
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder().build();

        assert_eq!(gleipnir.roots_of, String::default());
    }

    #[test]
    #[should_panic]
    fn should_panic_when_attribute_is_not_present() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder().build();

        assert_eq!(gleipnir.roots_of, String::default());
    }
}

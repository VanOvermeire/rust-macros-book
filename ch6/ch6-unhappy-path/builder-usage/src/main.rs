fn main() {}

#[cfg(test)]
mod tests {
    use  builder_macro::Builder;

    #[test]
    fn should_generate_builder_for_struct_with_no_properties() {
        #[derive(Builder)]
        struct ExampleStructNoFields {}

        let _: ExampleStructNoFields = ExampleStructNoFields::builder()
            .build();
    }

    #[test]
    fn should_generate_builder_for_struct_with_one_property() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
    }

    #[test]
    fn should_generate_builder_for_struct_with_two_properties() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1)
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
    }

    #[test]
    fn should_generate_builder_for_struct_with_multiple_properties() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
            other_necessities: Vec<String>,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1)
            .other_necessities(vec![
                "sound of cat's footsteps".to_string(),
                "beard of a woman".to_string(),
                "spittle of a bird".to_string(),
            ])
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
        assert_eq!(gleipnir.other_necessities.len(), 3);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_field_is_missing() {
        #[derive(Builder)]
        struct Gleipnir {
            _roots_of: String,
        }

        Gleipnir::builder().build();
    }
}

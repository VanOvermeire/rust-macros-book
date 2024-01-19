fn main() {}

#[cfg(test)]
mod tests {
    use builder_macro::Builder;

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
    fn should_generate_builder_for_struct_with_one_renamed_property() {
        #[derive(Builder)]
        struct Gleipnir {
            #[rename("tops_of")]
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder()
            .tops_of("mountains".to_string())
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
    }

    #[test]
    fn should_generate_builder_for_struct_with_two_properties_one_custom_name() {
        #[derive(Builder)]
        struct Gleipnir {
            #[rename("tops_of")]
            roots_of: String,
            breath_of_a_fish: u8,
        }

        let gleipnir = Gleipnir::builder()
            .tops_of("mountains".to_string())
            .breath_of_a_fish(1)
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
    }

    #[test]
    fn should_generate_builder_for_struct_with_two_properties_two_custom_name() {
        #[derive(Builder)]
        struct Gleipnir {
            #[rename("tops_of")]
            roots_of: String,
            #[rename("breath_of_man")]
            breath_of_a_fish: u8,
        }

        let gleipnir = Gleipnir::builder()
            .tops_of("mountains".to_string())
            .breath_of_man(1)
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
    }
}

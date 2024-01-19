use config_macro::config_struct;

#[config_struct]
#[derive(Debug)]
struct ConfigStruct {}

fn main() {
    let config = ConfigStruct::new();
    println!("{config:?}")
}

#[cfg(test)]
mod tests {
    use config_macro::{config,config_struct};

    #[test]
    fn should_generate_config_struct_with_expected_values() {
        config!();

        let cfg = Config::new();
        let user = cfg.0.get("user").unwrap();

        assert_eq!(user, "admin");
    }

    #[test]
    fn should_generate_config_for_existing_struct_with_expected_values() {
        #[config_struct(path = "./config-macro-usage/tests/configuration/config.yaml")]
        struct MyConfigStruct {}

        let cfg = MyConfigStruct::new();

        assert_eq!(cfg.user, "test");
    }
}

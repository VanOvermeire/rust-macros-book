use config_macro::config;

fn main() {
    config!();

    let cfg = Config::new();
    let user = cfg.0.get("user").unwrap();
    println!("{user}");
}

#[cfg(test)]
mod tests {
    use config_macro::config;

    #[test]
    fn should_generate_config_struct_with_expected_values() {
        config!();

        let cfg = Config::new();
        let user = cfg.0.get("user").unwrap();

        assert_eq!(user, "admin");
    }

    #[test]
    fn should_generate_config_struct_with_expected_values_for_path_override() {
        config!(path = "./config-macro-usage/tests/configuration/config.yaml");

        let cfg = Config::new();
        let user = cfg.0.get("user").unwrap();

        assert_eq!(user, "test");
    }
}
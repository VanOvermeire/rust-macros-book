use std::collections::HashMap;

use config_macro::config_struct;

#[config_struct]
#[derive(Debug)]
struct ConfigStruct {}

fn main() {
    let config = ConfigStruct::new();
    let map: HashMap<String, String> = config.into();
    println!("{map:?}");
}

pub fn format_config<T: for<'de> serde::Deserialize<'de>>(
    config: Option<&String>,
    default: T,
) -> T {
    match config {
        Some(config) => {
            let file = std::fs::read_to_string(config).expect("Cannot read the config file");

            if config.ends_with(".toml") {
                toml::from_str::<T>(&file).expect("Unable to convert the file")
            } else if config.ends_with(".json") {
                serde_json::from_str::<T>(&file).expect("Unable to convert the file")
            } else if config.ends_with(".yml") || config.ends_with(".yaml") {
                serde_yaml::from_str::<T>(&file).expect("Unable to convert the file")
            } else {
                panic!("Unknown extension")
            }
        }
        None => default,
    }
}

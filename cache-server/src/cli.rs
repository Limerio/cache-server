use clap::{Arg, ArgMatches, Command};
use serde::Deserialize;

pub fn init() -> Command {
    Command::new("cache-server")
        .bin_name("cache-server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .default_value("8080"),
        )
        .arg(Arg::new("config").short('c').long("config").required(false))
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: String,
}

pub fn format_config(config: Option<&String>, matches: ArgMatches) -> Config {
    match config {
        Some(config) => {
            let file = std::fs::read_to_string(config).expect("Cannot read the config file");

            if config.ends_with(".toml") {
                toml::from_str::<Config>(&file).expect("Unable to convert the file")
            } else if config.ends_with(".json") {
                serde_json::from_str::<Config>(&file).expect("Unable to convert the file")
            } else if config.ends_with(".yml") || config.ends_with(".yaml") {
                serde_yaml::from_str::<Config>(&file).expect("Unable to convert the file")
            } else {
                panic!("Unknown extension")
            }
        }
        None => Config {
            port: matches.get_one::<String>("port").unwrap().to_string(),
        },
    }
}

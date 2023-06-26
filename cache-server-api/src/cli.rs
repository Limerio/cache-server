use clap::{Arg, ArgMatches, Command};
use serde::Deserialize;

pub fn init() -> Command {
    Command::new("cache-server-api")
        .bin_name("cache-server-api")
        .arg(
            Arg::new("port-db")
                .alias("pdb")
                .long("port-db")
                .default_value("8080"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .default_value("8000"),
        )
        .arg(Arg::new("config").long("config").short('c'))
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: String,
    pub db: DBConfig,
}

#[derive(Deserialize, Debug)]
pub struct DBConfig {
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
        None => {
            let db = DBConfig {
                port: matches.get_one::<String>("port-db").unwrap().to_string(),
            };

            Config {
                port: matches.get_one::<String>("port").unwrap().to_string(),
                db,
            }
        }
    }
}

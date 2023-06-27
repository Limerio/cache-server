use clap::{Arg, Command};
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

#[derive(Deserialize)]
pub struct Config {
    pub port: String,
}

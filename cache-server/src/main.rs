mod db;
mod server;

use clap::{Arg, Command};
use db::Db;
use serde::Deserialize;
use std::fs;
use tokio::net::TcpListener;

fn cli() -> Command {
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
struct Config {
    port: String,
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    let db = Db::new();
    let config = matches.get_one::<String>("config");

    let config = if config.is_some() {
        let config = config.unwrap();
        let file = fs::read_to_string(config).expect("Cannot read the config file");

        if config.ends_with(".toml") {
            toml::from_str::<Config>(&file).expect("Unable to convert the file")
        } else if config.ends_with(".json") {
            serde_json::from_str::<Config>(&file).expect("Unable to convert the file")
        } else if config.ends_with(".yml") || config.ends_with(".yaml") {
            serde_yaml::from_str::<Config>(&file).expect("Unable to convert the file")
        } else {
            panic!("Unknown extension")
        }
    } else {
        Config {
            port: matches.get_one::<String>("port").unwrap().to_string(),
        }
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .expect("Unable to start the server");

    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                let mut db = db.clone();

                tokio::spawn(async move {
                    println!("Handle connection");
                    if let Err(err) = server::handle_connection(&mut stream, &mut db).await {
                        println!("Connection error");
                        println!("{:?}", err)
                    }
                });
            }
            Err(_e) => println!("Error"),
        }
    }
}

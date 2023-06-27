use std::time::Duration;

use cache_server_shared::Connection;
use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("ping")
        .about("Ping the server")
        .arg(
            Arg::new("infinite")
                .short('i')
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sleep")
                .long("sleep")
                .short('s')
                .default_value("0"),
        )
}

pub async fn subcommand(mut connection: Connection, sub_matches: &ArgMatches) {
    let infinite = sub_matches.get_one::<bool>("infinite");
    let sleep = sub_matches
        .get_one::<String>("sleep")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    if sleep > 0 && !infinite.unwrap() {
        println!("Impossible to do an interval if the parameter infinite isn't set");

        std::process::exit(1);
    }

    match infinite {
        Some(false) => println!("{}", connection.ping().await),
        Some(true) => loop {
            println!("{}", connection.ping().await);
            if sleep > 0 {
                tokio::time::sleep(Duration::from_millis(sleep)).await;
            }
        },
        None => unreachable!("Strange thing"),
    }
}

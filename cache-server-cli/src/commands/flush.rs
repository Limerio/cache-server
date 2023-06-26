use clap::{ArgMatches, Command};

use cache_server_client;

pub fn cmd() -> Command {
    Command::new("flush")
        .short_flag('f')
        .about("Flush data in the database")
}

pub async fn subcommand(matches: ArgMatches) {
    let port = matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::flush(port.to_string()).await;

    println!("{}", response);
}

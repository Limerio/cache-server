use clap::{Arg, ArgMatches, Command};

use cache_server_client;

pub fn cmd() -> Command {
    Command::new("all")
        .short_flag('a')
        .about("Get all data in the database")
        .arg(Arg::new("port").long("port").default_value("8080"))
}

pub async fn subcommand(sub_matches: &ArgMatches) {
    let port = sub_matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::all(port.to_string()).await;

    println!("{}", response);
}

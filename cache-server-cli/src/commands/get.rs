use cache_server_shared::Connection;
use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("get")
        .about("Get data in the database")
        .arg(Arg::new("key").required(true))
}

pub async fn subcommand(mut connection: Connection, sub_matches: &ArgMatches) {
    let key = sub_matches
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");

    let response = connection.get(key.to_string()).await;

    println!("{}", response);
}

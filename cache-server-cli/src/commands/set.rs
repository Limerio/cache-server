use cache_server_shared::Connection;
use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("set")
        .short_flag('s')
        .about("Add data in the database")
        .arg(Arg::new("key").required(true))
        .arg(Arg::new("value").required(true))
}

pub async fn subcommand(mut connection: Connection, sub_matches: &ArgMatches) {
    let key = sub_matches
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");
    let value = sub_matches
        .get_one::<String>("value")
        .expect("Cannot get the parameter \"value\"");

    let response = connection.set(key.to_string(), value.to_string()).await;

    println!("{}", response);
}

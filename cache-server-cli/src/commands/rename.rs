use cache_server_shared::Connection;
use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("rename")
        .about("Add data in the database")
        .arg(Arg::new("old_key").required(true))
        .arg(Arg::new("new_key").required(true))
}

pub async fn subcommand(mut connection: Connection, sub_matches: &ArgMatches) {
    let old_key = sub_matches
        .get_one::<String>("old_key")
        .expect("Cannot get the parameter \"old_key\"");
    let new_key = sub_matches
        .get_one::<String>("new_key")
        .expect("Cannot get the parameter \"new_key\"");

    let response = connection
        .rename(old_key.to_string(), new_key.to_string())
        .await;

    println!("{}", response);
}

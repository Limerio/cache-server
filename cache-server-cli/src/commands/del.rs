use clap::{Arg, ArgMatches, Command};

use cache_server_client;

pub fn cmd() -> Command {
    Command::new("del")
        .short_flag('d')
        .about("Delete data in the database")
        .arg(Arg::new("key").required(true))
}

pub async fn subcommand(matches: ArgMatches) {
    let sub_matches = matches.subcommand_matches(cmd().get_name()).unwrap();
    let key = sub_matches
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");

    let port = matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::del(port.to_string(), key.to_string()).await;

    println!("{}", response);
}

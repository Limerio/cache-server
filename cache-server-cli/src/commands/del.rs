use clap::{Arg, ArgMatches, Command};

use cache_server_client;

pub fn cmd() -> Command {
    Command::new("del")
        .short_flag('d')
        .about("Delete data in the database")
        .arg(Arg::new("key").required(true))
        .arg(Arg::new("port").long("port").default_value("8080"))
}

pub async fn subcommand(sub_matches: &ArgMatches) {
    let key = sub_matches
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");

    let port = sub_matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::del(port.to_string(), key.to_string()).await;

    println!("{}", response);
}

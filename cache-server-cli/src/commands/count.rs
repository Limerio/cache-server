use clap::{Arg, ArgMatches, Command};

use cache_server_client;

pub fn cmd() -> Command {
    Command::new("count")
        .short_flag('c')
        .about("Get a count of all data in the database")
        .arg(Arg::new("port").long("port").default_value("8080"))
}

pub async fn subcommand(sub_matches: &ArgMatches) {
    let port = sub_matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::count(port.to_string()).await;

    println!("{}", response);
}

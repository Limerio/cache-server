use clap::{Arg, ArgMatches, Command};

use cache_server_client;

pub fn cmd() -> Command {
    Command::new("flush")
        .short_flag('f')
        .about("Flush data in the database")
        .arg(Arg::new("port").long("port").default_value("8080"))
}

pub async fn subcommand(sub_matches: &ArgMatches) {
    let port = sub_matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::flush(port.to_string()).await;

    println!("{}", response);
}

use clap::{ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("all")
        .short_flag('a')
        .about("Get all data in the database")
}

pub async fn subcommand(matches: ArgMatches) {
    let port = matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::all(port.to_string()).await;

    println!("{}", response);
}

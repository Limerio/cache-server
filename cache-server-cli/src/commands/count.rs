use clap::{ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("count")
        .short_flag('c')
        .about("Get a count of all data in the database")
}

pub async fn subcommand(matches: ArgMatches) {
    let port = matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::count(port.to_string()).await;

    println!("{}", response);
}

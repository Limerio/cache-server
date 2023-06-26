use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("get")
        .short_flag('g')
        .about("Get data in the database")
        .arg(Arg::new("key").required(true))
}

pub async fn subcommand(matches: ArgMatches) {
    let key = matches
        .subcommand_matches(cmd().get_name())
        .unwrap()
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");
    let port = matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::get(port.to_string(), key.to_string()).await;

    println!("{}", response);
}

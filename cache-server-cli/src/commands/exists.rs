use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("exists")
        .short_flag('e')
        .about("Check if a key exists in the database")
        .arg(Arg::new("key").required(true))
}

pub async fn subcommand(matches: ArgMatches) {
    let sub_matches = matches.subcommand_matches(cmd().get_name()).unwrap();

    let key = sub_matches
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");
    let port = matches.get_one::<String>("port").unwrap();

    let response = cache_server_client::exists(port.to_string(), key.to_string()).await;

    println!("{}", response);
}

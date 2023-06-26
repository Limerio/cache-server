use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("set")
        .short_flag('s')
        .about("Add data in the database")
        .arg(Arg::new("key").required(true))
        .arg(Arg::new("value").required(true))
}

pub async fn subcommand(matches: ArgMatches) {
    let sub_matches = matches.subcommand_matches(cmd().get_name()).unwrap();
    let key = sub_matches
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");
    let value = sub_matches
        .get_one::<String>("value")
        .expect("Cannot get the parameter \"value\"");

    let port = matches.get_one::<String>("port").unwrap();

    let response =
        cache_server_client::set(port.to_string(), key.to_string(), value.to_string()).await;

    println!("{}", response);
}

use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("set")
        .short_flag('s')
        .about("Add data in the database")
        .arg(Arg::new("key").required(true))
        .arg(Arg::new("value").required(true))
        .arg(Arg::new("port").long("port").default_value("8080"))
}

pub async fn subcommand(sub_matches: &ArgMatches) {
    let key = sub_matches
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");
    let value = sub_matches
        .get_one::<String>("value")
        .expect("Cannot get the parameter \"value\"");
    let port = sub_matches.get_one::<String>("port").unwrap();

    let response =
        cache_server_client::set(port.to_string(), key.to_string(), value.to_string()).await;

    println!("{}", response);
}

use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("rename")
        .short_flag('r')
        .about("Add data in the database")
        .arg(Arg::new("old_key").required(true))
        .arg(Arg::new("new_key").required(true))
        .arg(Arg::new("port").long("port").default_value("8080"))
}

pub async fn subcommand(sub_matches: &ArgMatches) {
    let old_key = sub_matches
        .get_one::<String>("old_key")
        .expect("Cannot get the parameter \"old_key\"");
    let new_key = sub_matches
        .get_one::<String>("new_key")
        .expect("Cannot get the parameter \"new_key\"");
    let port = sub_matches.get_one::<String>("port").unwrap();

    let response =
        cache_server_client::rename(port.to_string(), old_key.to_string(), new_key.to_string())
            .await;

    println!("{}", response);
}

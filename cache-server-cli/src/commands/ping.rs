use cache_server_shared::Connection;
use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("ping")
        .short_flag('p')
        .about("Ping the server")
        .arg(
            Arg::new("infinite")
                .short('t')
                .action(clap::ArgAction::SetTrue),
        )
}

pub async fn subcommand(mut connection: Connection, sub_matches: &ArgMatches) {
    let infinite = sub_matches.get_one::<bool>("infinite");

    match infinite {
        Some(false) => println!("{}", connection.ping().await),
        Some(true) => loop {
            println!("{}", connection.ping().await)
        },
        None => unreachable!("Strange thing"),
    }
}

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
        Some(false) => ping_write(&mut connection).await,
        Some(true) => loop {
            ping_write(&mut connection).await
        },
        None => unreachable!("Strange thing"),
    }
}

async fn ping_write(connection: &mut Connection) {
    connection.write("PING".to_string()).await;

    let response = connection.read().await;

    println!("{}", response)
}

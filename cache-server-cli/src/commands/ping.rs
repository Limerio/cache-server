use cache_server_client::Connection;
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
        .arg(Arg::new("port").long("port").default_value("8080"))
}

pub async fn subcommand(sub_matches: &ArgMatches) {
    let infinite = sub_matches.get_one::<bool>("infinite");
    let port = sub_matches.get_one::<String>("port").unwrap();

    let mut listener = Connection::new(port.to_string()).await;

    match infinite {
        Some(false) => ping_write(&mut listener).await,
        Some(true) => loop {
            ping_write(&mut listener).await
        },
        None => unreachable!("Strange thing"),
    }
}

async fn ping_write(listener: &mut Connection) {
    listener.write("PING".to_string()).await;

    let response = listener.read().await;

    println!("{}", response)
}

use clap::{Arg, ArgMatches, Command};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::get_response;

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

pub async fn subcommand(sub_matches: &ArgMatches) {
    let infinite = sub_matches.get_one::<bool>("infinite");

    let mut listener = TcpStream::connect("127.0.0.1:8080")
        .await
        .expect("Unable to connect to the database");

    match infinite {
        Some(false) => ping(&mut listener).await,
        Some(true) => loop {
            ping(&mut listener).await
        },
        None => unreachable!("Strange thing"),
    }
}

async fn ping(listener: &mut TcpStream) {
    listener.write_all("PING".as_bytes()).await.unwrap();

    let response = get_response(listener).await;

    println!("{}", response);
}

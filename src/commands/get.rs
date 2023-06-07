use clap::{Arg, ArgMatches, Command};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::get_response;

pub fn cmd() -> Command {
    Command::new("get")
        .short_flag('g')
        .about("Get data in the database")
        .arg(Arg::new("key").required(true))
}

pub async fn subcommand(sub_matches: &ArgMatches) {
    let key = sub_matches
        .get_one::<String>("key")
        .expect("Cannot get the parameter \"key\"");

    let mut listener = TcpStream::connect("127.0.0.1:8080")
        .await
        .expect("Unable to connect to the database");

    listener
        .write_all(format!("GET {}", key).as_bytes())
        .await
        .unwrap();

    let response = get_response(&mut listener).await;

    println!("{}", response);
}

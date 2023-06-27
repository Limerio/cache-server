use cache_server_shared::Connection;
use clap::Command;

pub fn cmd() -> Command {
    Command::new("all")
        .short_flag('a')
        .about("Get all data in the database")
}

pub async fn subcommand(mut connection: Connection) {
    let response = connection.all().await;

    println!("{}", response);
}

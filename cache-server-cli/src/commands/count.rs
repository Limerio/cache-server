use cache_server_shared::Connection;
use clap::Command;

pub fn cmd() -> Command {
    Command::new("count").about("Get a count of all data in the database")
}

pub async fn subcommand(mut connection: Connection) {
    let response = connection.count().await;

    println!("{}", response);
}

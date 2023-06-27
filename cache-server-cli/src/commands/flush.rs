use clap::Command;

use cache_server_shared::Connection;

pub fn cmd() -> Command {
    Command::new("flush")
        .short_flag('f')
        .about("Flush data in the database")
}

pub async fn subcommand(mut connection: Connection) {
    let response = connection.flush().await;

    println!("{}", response);
}

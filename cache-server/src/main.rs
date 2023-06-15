mod db;
mod server;

use clap::{Arg, Command};
use db::Db;
use tokio::net::TcpListener;

fn cli() -> Command {
    Command::new("cache-server").bin_name("cache-server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .default_value("8080"),
    )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    let db = Db::new();
    let port = matches.get_one::<String>("port").unwrap();

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Unable to start the server");

    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                let mut db = db.clone();

                tokio::spawn(async move {
                    println!("Handle connection");
                    if let Err(err) = server::handle_connection(&mut stream, &mut db).await {
                        println!("Connection error");
                        println!("{:?}", err)
                    }
                });
            }
            Err(_e) => println!("Error"),
        }
    }
}

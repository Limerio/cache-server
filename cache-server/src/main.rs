mod cli;
mod db;
mod server;

use db::Db;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let matches = cli::init().get_matches();
    let db = Db::new();
    let config = matches.get_one::<String>("config");

    let config = cli::format_config(config, matches.clone());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
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

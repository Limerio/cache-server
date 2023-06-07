use cache_server::{server, Db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let db = Db::new();

    let listener = TcpListener::bind("0.0.0.0:8080")
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

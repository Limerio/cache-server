use cache_server_shared::Connection;

#[delete("/")]
pub async fn route() -> String {
    let mut connection = Connection::new(std::env::var("SERVER_PORT").unwrap()).await;

    connection.flush().await
}

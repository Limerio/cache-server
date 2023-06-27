use cache_server_shared::Connection;

#[get("/<key>/exists")]
pub async fn route(key: String) -> String {
    let mut connection = Connection::new(std::env::var("SERVER_PORT").unwrap()).await;

    connection.exists(key).await
}

use cache_server_shared::Connection;

#[get("/<key>")]
pub async fn route(key: String) -> String {
    let mut connection = Connection::new(std::env::var("SERVER_PORT").unwrap()).await;

    connection.get(key).await
}

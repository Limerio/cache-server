use cache_server_shared::Connection;

#[get("/")]
pub async fn route() -> String {
    let mut connection = Connection::new(std::env::var("SERVER_PORT").unwrap()).await;

    connection.all().await
}

#[get("/ping")]
pub async fn route() -> String {
    cache_server_client::ping(std::env::var("SERVER_PORT").unwrap()).await
}

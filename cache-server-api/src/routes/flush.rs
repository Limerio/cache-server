#[delete("/")]
pub async fn route() -> String {
    cache_server_client::flush(std::env::var("SERVER_PORT").unwrap()).await
}

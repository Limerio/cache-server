#[get("/<key>/exists")]
pub async fn route(key: String) -> String {
    cache_server_client::exists(std::env::var("SERVER_PORT").unwrap(), key).await
}

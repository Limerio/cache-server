#[get("/<key>")]
pub async fn route(key: String) -> String {
    cache_server_client::get(std::env::var("SERVER_PORT").unwrap(), key).await
}

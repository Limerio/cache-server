#[get("/")]
pub async fn route() -> String {
    cache_server_client::all(std::env::var("SERVER_PORT").unwrap()).await
}

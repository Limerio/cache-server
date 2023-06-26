#[get("/count")]
pub async fn route() -> String {
    cache_server_client::count(std::env::var("SERVER_PORT").unwrap()).await
}

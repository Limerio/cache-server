use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct KeyValue {
    key: String,
    value: String,
}

#[post("/", data = "<data>")]
pub async fn route(data: Json<KeyValue>) -> String {
    cache_server_client::set(
        std::env::var("SERVER_PORT").unwrap(),
        data.key.to_string(),
        data.value.to_string(),
    )
    .await
}

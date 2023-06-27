use cache_server_shared::Connection;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct KeyValue {
    key: String,
    value: String,
}

#[post("/", data = "<data>")]
pub async fn route(data: Json<KeyValue>) -> String {
    let mut connection = Connection::new(std::env::var("SERVER_PORT").unwrap()).await;

    connection
        .set(data.key.to_string(), data.value.to_string())
        .await
}

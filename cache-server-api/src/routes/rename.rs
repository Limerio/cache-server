use cache_server_shared::Connection;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewKey {
    new_key: String,
}

#[put("/<key>/rename", data = "<data>")]
pub async fn route(key: String, data: Json<NewKey>) -> String {
    let mut connection = Connection::new(std::env::var("SERVER_PORT").unwrap()).await;

    connection.rename(key, data.new_key.to_string()).await
}

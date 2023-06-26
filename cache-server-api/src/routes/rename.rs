use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewKey {
    new_key: String,
}

#[put("/<key>/rename", data = "<data>")]
pub async fn route(key: String, data: Json<NewKey>) -> String {
    cache_server_client::rename(
        std::env::var("SERVER_PORT").unwrap(),
        key,
        data.new_key.clone(),
    )
    .await
}

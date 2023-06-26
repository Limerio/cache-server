#[macro_use]
extern crate rocket;

mod cli;

use cli::cli;
use rocket::serde::{json::Json, Deserialize};
use std::{env, fs};

#[get("/ping")]
async fn ping() -> String {
    cache_server_client::ping(env::var("SERVER_PORT").unwrap()).await
}

#[get("/")]
async fn all() -> String {
    cache_server_client::all(env::var("SERVER_PORT").unwrap()).await
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct KeyValue {
    key: String,
    value: String,
}

#[post("/", data = "<data>")]
async fn set(data: Json<KeyValue>) -> String {
    cache_server_client::set(
        env::var("SERVER_PORT").unwrap(),
        data.key.to_string(),
        data.value.to_string(),
    )
    .await
}

#[get("/count")]
async fn count() -> String {
    cache_server_client::count(env::var("SERVER_PORT").unwrap()).await
}

#[get("/<key>")]
async fn get(key: String) -> String {
    cache_server_client::get(env::var("SERVER_PORT").unwrap(), key).await
}

#[get("/<key>/exists")]
async fn exists(key: String) -> String {
    cache_server_client::exists(env::var("SERVER_PORT").unwrap(), key).await
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewKey {
    new_key: String,
}

#[put("/<key>/rename", data = "<data>")]
async fn rename(key: String, data: Json<NewKey>) -> String {
    cache_server_client::rename(env::var("SERVER_PORT").unwrap(), key, data.new_key.clone()).await
}

#[delete("/<key>")]
async fn del(key: String) -> String {
    cache_server_client::del(env::var("SERVER_PORT").unwrap(), key).await
}

#[delete("/")]
async fn flush() -> String {
    cache_server_client::flush(env::var("SERVER_PORT").unwrap()).await
}

#[derive(Deserialize, Debug)]
struct Config {
    port: String,
    port_db: String,
}

#[launch]
async fn rocket() -> _ {
    let matches = cli().get_matches();

    let config = matches.get_one::<String>("config");

    let config = if config.is_some() {
        let config = config.unwrap();
        let file = fs::read_to_string(config).expect("Cannot read the config file");

        if config.ends_with(".toml") {
            toml::from_str::<Config>(&file).expect("Unable to convert the file")
        } else if config.ends_with(".json") {
            serde_json::from_str::<Config>(&file).expect("Unable to convert the file")
        } else if config.ends_with(".yml") || config.ends_with(".yaml") {
            serde_yaml::from_str::<Config>(&file).expect("Unable to convert the file")
        } else {
            panic!("Unknown extension")
        }
    } else {
        Config {
            port: matches.get_one::<String>("port").unwrap().to_string(),
            port_db: matches.get_one::<String>("port-db").unwrap().to_string(),
        }
    };

    cache_server_client::ping(config.port_db.to_string()).await;

    env::set_var("SERVER_PORT", config.port_db);
    env::set_var("ROCKET_PORT", config.port);

    rocket::build().mount(
        "/",
        routes![set, get, del, all, count, exists, flush, rename, ping],
    )
}

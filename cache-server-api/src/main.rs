#[macro_use]
extern crate rocket;

mod cli;
mod routes;

#[launch]
async fn rocket() -> _ {
    let matches = cli::init().get_matches();

    let config = matches.get_one::<String>("config");

    let config = cli::format_config(config, matches.clone());

    cache_server_client::ping(config.db.port.to_string()).await;

    std::env::set_var("SERVER_PORT", config.db.port);
    std::env::set_var("ROCKET_PORT", config.port);

    rocket::build().mount(
        "/",
        routes![
            routes::set::route,
            routes::get::route,
            routes::del::route,
            routes::all::route,
            routes::count::route,
            routes::exists::route,
            routes::flush::route,
            routes::rename::route,
            routes::ping::route
        ],
    )
}

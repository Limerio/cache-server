use cache_server_shared::Connection;

#[macro_use]
extern crate rocket;

mod cli;
mod routes;

#[launch]
async fn rocket() -> _ {
    let matches = cli::init().get_matches();

    let config = matches.get_one::<String>("config");

    let db = cli::DBConfig {
        port: matches.get_one::<String>("port-db").unwrap().to_string(),
    };

    let config = cache_server_shared::format_config::<cli::Config>(
        config,
        cli::Config {
            port: matches.get_one::<String>("port").unwrap().to_string(),
            db,
        },
    );

    let mut connection = Connection::new(config.db.port.clone()).await;

    connection.ping().await;

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

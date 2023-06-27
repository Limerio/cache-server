use cache_server_shared::Connection;

mod cli;
mod commands;

#[tokio::main]
async fn main() {
    let matches = cli::cmds().get_matches();

    let config = matches.get_one::<String>("config");

    let default_config = cli::Config {
        port: matches.get_one::<String>("port").unwrap().to_string(),
    };

    let config = cache_server_shared::format_config::<cli::Config>(config, default_config);

    let connection = Connection::new(config.port).await;

    match matches.subcommand() {
        Some(("get", sub_matches)) => commands::get::subcommand(connection, sub_matches).await,
        Some(("set", sub_matches)) => commands::set::subcommand(connection, sub_matches).await,
        Some(("del", sub_matches)) => commands::del::subcommand(connection, sub_matches).await,
        Some(("ping", sub_matches)) => commands::ping::subcommand(connection, sub_matches).await,
        Some(("all", _sub_matches)) => commands::all::subcommand(connection).await,
        Some(("count", _sub_matches)) => commands::count::subcommand(connection).await,
        Some(("exists", sub_matches)) => {
            commands::exists::subcommand(connection, sub_matches).await
        }
        Some(("flush", _sub_matches)) => commands::flush::subcommand(connection).await,
        Some(("rename", sub_matches)) => {
            commands::rename::subcommand(connection, sub_matches).await
        }
        _ => unreachable!("Strange things"),
    }
}

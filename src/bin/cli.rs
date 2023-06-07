use cache_server::cli;
use cache_server::commands;

#[tokio::main]
async fn main() {
    let matches = cli::cmds().get_matches();

    match matches.subcommand() {
        Some(("get", sub_matches)) => commands::get::subcommand(sub_matches).await,
        Some(("set", sub_matches)) => commands::set::subcommand(sub_matches).await,
        Some(("ping", sub_matches)) => commands::ping::subcommand(sub_matches).await,
        _ => unreachable!("Strange things"),
    }
}

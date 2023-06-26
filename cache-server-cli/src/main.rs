mod cli;
mod commands;

#[tokio::main]
async fn main() {
    let matches = cli::cmds().get_matches();

    match matches.subcommand() {
        Some(("get", _sub_matches)) => commands::get::subcommand(matches).await,
        Some(("set", _sub_matches)) => commands::set::subcommand(matches).await,
        Some(("del", _sub_matches)) => commands::del::subcommand(matches).await,
        Some(("ping", _sub_matches)) => commands::ping::subcommand(matches).await,
        Some(("all", _sub_matches)) => commands::all::subcommand(matches).await,
        Some(("count", _sub_matches)) => commands::count::subcommand(matches).await,
        Some(("exists", _sub_matches)) => commands::exists::subcommand(matches).await,
        Some(("flush", _sub_matches)) => commands::flush::subcommand(matches).await,
        Some(("rename", _sub_matches)) => commands::rename::subcommand(matches).await,
        _ => unreachable!("Strange things"),
    }
}

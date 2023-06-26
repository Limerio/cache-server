mod cli;
mod commands;

#[tokio::main]
async fn main() {
    let matches = cli::cmds().get_matches();

    match matches.subcommand() {
        Some(("get", sub_matches)) => commands::get::subcommand(sub_matches).await,
        Some(("set", sub_matches)) => commands::set::subcommand(sub_matches).await,
        Some(("del", sub_matches)) => commands::del::subcommand(sub_matches).await,
        Some(("ping", sub_matches)) => commands::ping::subcommand(sub_matches).await,
        Some(("all", sub_matches)) => commands::all::subcommand(sub_matches).await,
        Some(("count", sub_matches)) => commands::count::subcommand(sub_matches).await,
        Some(("exists", sub_matches)) => commands::exists::subcommand(sub_matches).await,
        Some(("flush", sub_matches)) => commands::flush::subcommand(sub_matches).await,
        Some(("rename", sub_matches)) => commands::rename::subcommand(sub_matches).await,
        _ => unreachable!("Strange things"),
    }
}

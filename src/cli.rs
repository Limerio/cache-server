use clap::Command;

use crate::commands;

pub fn cmds() -> Command {
    Command::new("cache-server")
        .about("Small project similaire to Redis")
        .author("Limerio")
        .bin_name("cache-server-cli")
        .subcommand_required(true)
        .subcommand(commands::set::cmd())
        .subcommand(commands::get::cmd())
        .subcommand(commands::ping::cmd())
}

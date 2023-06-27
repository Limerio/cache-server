use clap::{Arg, Command};
use serde::Deserialize;

use crate::commands;

pub fn cmds() -> Command {
    Command::new("cache-server-cli")
        .about("Small project similaire to Redis")
        .author("Limerio")
        .bin_name("cache-server-cli")
        .subcommand_required(true)
        .subcommand(commands::set::cmd())
        .subcommand(commands::get::cmd())
        .subcommand(commands::del::cmd())
        .subcommand(commands::ping::cmd())
        .subcommand(commands::all::cmd())
        .subcommand(commands::count::cmd())
        .subcommand(commands::exists::cmd())
        .subcommand(commands::flush::cmd())
        .subcommand(commands::rename::cmd())
        .arg(Arg::new("port").long("port").default_value("8080"))
}

#[derive(Deserialize)]
pub struct Config {
    pub port: String,
}

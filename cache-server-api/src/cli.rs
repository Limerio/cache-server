use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("cache-server-api")
        .bin_name("cache-server-api")
        .arg(
            Arg::new("port-db")
                .alias("pdb")
                .long("port-db")
                .default_value("8080"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .default_value("8000"),
        )
        .arg(Arg::new("config").long("config").short('c'))
}

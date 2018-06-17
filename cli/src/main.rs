extern crate clap;
extern crate carina_core;
extern crate carina_core_protocol;
#[macro_use]
extern crate log;
extern crate loggify;
#[macro_use]
extern crate prettytable;

mod commands;

use clap::{App, Arg, SubCommand};

fn main() {
    loggify::Loggify::init_with_level(log::Level::Debug).unwrap();

        let matches = App::new("Carina network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool for carina")
        .subcommand(
            SubCommand::with_name("misc")
            .about("Pings every peer.")
            .arg(Arg::with_name("CONFIG")
                .value_name("config")
                .help("Sets the location of the config file.")
                .takes_value(true)
                .long("config")
                .default_value("./config.yml"))
            )
        .subcommand(
            SubCommand::with_name("console")
            .about("Actual implementation.")
            .arg(Arg::with_name("CONFIG")
                .value_name("config")
                .help("Sets the location of the config file.")
                .takes_value(true)
                .long("config")
                .default_value("./config.yml"))
            )
        .get_matches();

    match matches.subcommand() {
        ("misc", Some(sub_matches))    => commands::ping::execute(sub_matches),
        ("console", Some(sub_matches)) => commands::console::execute(sub_matches),
        _                              => error!("Not valid")
    }
}

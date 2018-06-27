#![deny(
    missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications, warnings
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! Terminal cli tool for carina

extern crate base64;
extern crate clap;
extern crate carina_core;
extern crate carina_core_protocol;
#[macro_use]
extern crate log;
extern crate loggify;
#[macro_use]
extern crate prettytable;
extern crate protocol_builder_parser;
extern crate sodiumoxide;

mod console;
mod misc;

use clap::{App, Arg, SubCommand};

fn main() {
    loggify::Loggify::init_with_level(log::Level::Debug).unwrap();

    let matches = App::new("Carina network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool for carina")
        .subcommand(
            SubCommand::with_name("misc")
                .about("Misc commands")
                .subcommand(
                    SubCommand::with_name("content")
                        .about("Adds new content to the next block.")
                        .arg(Arg::with_name("CONFIG")
                            .value_name("config")
                            .help("Sets the location of the config file.")
                            .takes_value(true)
                            .long("config")
                            .default_value("./config.yml"))
                        .arg(Arg::with_name("CONTENT")
                            .help("Sets the content.")
                            .takes_value(true)
                            .required(true))
                )
                .subcommand(
                    SubCommand::with_name("ping")
                        .about("Pings all peers and checks if they are answer.")
                        .arg(Arg::with_name("CONFIG")
                        .value_name("config")
                        .help("Sets the location of the config file.")
                        .takes_value(true)
                        .long("config")
                        .default_value("./config.yml"))
                )
                .subcommand(
                    SubCommand::with_name("genkey")
                        .about("Generates a new secret key")
                )
                .subcommand(
                    SubCommand::with_name("pubkey")
                        .about("Generates a new public key from a secret key")
                        .arg(Arg::with_name("secret key").required(true))
                )
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
        ("misc", Some(sub_matches))    => misc::execute(sub_matches),
        ("console", Some(sub_matches)) => console::execute(sub_matches),
        _                              => error!("Not valid")
    }
}

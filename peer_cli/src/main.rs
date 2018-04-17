#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! Terminal client application for a peer
extern crate carina_logging;
extern crate carina_peer;
extern crate clap;
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod configuration;
mod console;

use carina_logging::LogBuilder;
use clap::{Arg, App, SubCommand};

fn main() {
    LogBuilder::new()
        .set_level(log::Level::Trace)
        .build()
        .unwrap();

    let matches = App::new("Carina network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool for carina")
        .subcommand(
            SubCommand::with_name("console")
            .arg(Arg::with_name("CONFIG")
                .value_name("config")
                .help("Sets the location of the config file.")
                .takes_value(true)
                .long("config")
                .default_value("./config.yml"))
            )
        .get_matches();

    match matches.subcommand() {
        ("console", Some(sub_matches)) => console::execute(sub_matches),
        (_, _)                         => panic!("No valid command!")
    }
}
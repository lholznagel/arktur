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

use carina_logging::LogBuilder;
use carina_peer::config::HolePuncher;
use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use configuration::Configuration;

fn main() {
    LogBuilder::new()
        .set_level(log::Level::Trace)
        .build()
        .unwrap();

    let matches = App::new("Carina network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool for carina")
        .arg(Arg::with_name("HOLE_PUNCHER_IP")
            .value_name("ip")
            .help("Sets the IP of the Hole puncher service")
            .takes_value(true)
            .required(true)
            .long("puncher_ip")
            .default_value("0.0.0.0"))
        .arg(Arg::with_name("HOLE_PUNCHER_PORT")
            .value_name("port")
            .help("Sets the port of the Hole puncher service.")
            .takes_value(true)
            .long("puncher_port")
            .default_value("50000"))
        .arg(Arg::with_name("CONFIG")
            .value_name("config")
            .help("Sets the location of the config file.")
            .takes_value(true)
            .long("config")
            .default_value("./config.yml"))
        .get_matches();

    let mut file = File::open(matches.value_of("CONFIG").unwrap().to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Configuration = serde_yaml::from_str(&content).unwrap();

    let mut config = config.to_config();
    config.hole_puncher = HolePuncher {
        host: matches.value_of("HOLE_PUNCHER_IP").unwrap().to_string(),
        port: matches.value_of("HOLE_PUNCHER_PORT").unwrap().parse().unwrap()
    };

    carina_peer::init(config);
}
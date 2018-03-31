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
extern crate carina_peer;
extern crate clap;

use carina_peer::config::{Config, HolePuncher};
use clap::{Arg, App};

fn main() {
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
        .arg(Arg::with_name("STORAGE")
            .value_name("storage")
            .help("Sets the location for the blocks.")
            .takes_value(true)
            .long("storage")
            .default_value("block_data"))
        .get_matches();

    let config = Config {
        hole_puncher: HolePuncher {
            host: matches.value_of("HOLE_PUNCHER_IP").unwrap().to_string(),
            port: matches.value_of("HOLE_PUNCHER_PORT").unwrap().parse().unwrap()
        },
        port: 0,
        storage: matches.value_of("STORAGE").unwrap().to_string()
    };

    carina_peer::init(config);
}
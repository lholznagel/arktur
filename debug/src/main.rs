#![deny(missing_docs)]

//! Small debug application
extern crate blockchain_hooks;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_network;
extern crate blockchain_protocol;
extern crate clap;

use clap::{App, Arg, SubCommand};

mod explore;

fn main() {
    let matches = App::new("Blockchain debug cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Debug tool for rust-blockchain")
        .arg(Arg::with_name("HOLE_PUNCHER_IP")
            .value_name("ip")
            .help("Sets the IP of the Hole puncher service")
            .takes_value(true)
            .long("puncher_ip")
            .default_value("0.0.0.0"))
        .arg(Arg::with_name("HOLE_PUNCHER_PORT")
            .value_name("port")
            .help("Sets the port of the Hole puncher service.")
            .takes_value(true)
            .long("puncher_port")
            .default_value("50000"))
        .subcommand(SubCommand::with_name("explore")
            .about("Checks if all peers know each other."))
        .get_matches();

    let mut hole_puncher = String::from("");
    hole_puncher.push_str(matches.value_of("HOLE_PUNCHER_IP").unwrap());
    hole_puncher.push_str(":");
    hole_puncher.push_str(matches.value_of("HOLE_PUNCHER_PORT").unwrap());

    match matches.subcommand() {
        ("explore", Some(sub_matches)) => explore::execute(hole_puncher, sub_matches),
        (_, _) => error!("Nothing to do")
    };
}
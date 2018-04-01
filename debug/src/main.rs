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

//! Small debug application
extern crate carina_hooks;
#[macro_use]
extern crate log;
extern crate carina_protocol;
extern crate clap;
extern crate futures_cpupool;
extern crate rand;

use clap::{App, Arg, SubCommand};

mod block;
mod explore;

fn main() {
    let matches = App::new("Carina debug cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Debug tool for Carina")
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
            .about("Explores the p2p network.")
            .arg(Arg::with_name("WAIT")
                .value_name("wait")
                .help("Sets the time to wait in seconds")
                .takes_value(true)
                .long("wait")
                .short("w")
                .default_value("30")))
        .subcommand(SubCommand::with_name("block")
            .about("Everything for blocks.")
            .arg(Arg::with_name("PEER_IP")
                .value_name("ip")
                .help("Sets the IP to a peer")
                .takes_value(true)
                .long("peer_ip")
                .default_value("0.0.0.0"))
            .arg(Arg::with_name("PEER_PORT")
                .value_name("port")
                .help("Sets the port to a peer.")
                .takes_value(true)
                .short("p")
                .long("peer_port")
                .required(true))
            .arg(Arg::with_name("MESSAGE")
                .value_name("message")
                .help("Message to send.")
                .takes_value(true)
                .short("m")
                .long("message")))
        .get_matches();

    let mut hole_puncher = String::from("");
    hole_puncher.push_str(matches.value_of("HOLE_PUNCHER_IP").unwrap());
    hole_puncher.push_str(":");
    hole_puncher.push_str(matches.value_of("HOLE_PUNCHER_PORT").unwrap());

    match matches.subcommand() {
        ("explore", Some(sub_matches)) => explore::execute(hole_puncher, sub_matches),
        ("block", Some(sub_matches)) => block::execute(sub_matches),
        (_, _) => error!("Nothing to do")
    };
}
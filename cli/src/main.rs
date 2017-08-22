extern crate toml;
#[macro_use]
extern crate serde_derive;

extern crate openssl;
extern crate clap;

pub mod models;
pub mod file;
pub mod keypair;

use clap::{App, Arg, SubCommand};
use models::config::Config;

fn main() {
    let matches = App::new("Rust-Blockchain Client")
        .version("0.1.0")
        .subcommand(SubCommand::with_name("init")
            .about("Initializes a client")
            .arg(Arg::with_name("user")
                .short("u")
                .long("user")
                .help("Name of the user")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("node")
                .short("n")
                .long("node")
                .help("Adress of the node")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("keypair")
                .short("k")
                .long("keypair")
                .help("Name of the key")
                .takes_value(true)
                .default_value("keypair")
            )
        )
        .get_matches();

    if let Some(init) = matches.subcommand_matches("init") {
        let dir = String::from("config");

        file::create_dir(&dir);
        keypair::generate_key_pair(String::from(init.value_of("keypair").unwrap()), &dir);

        let config = Config {
            user: String::from(init.value_of("user").unwrap()),
            node: String::from(init.value_of("node").unwrap()),
            keypair: String::from(init.value_of("keypair").unwrap())
        };

        file::save_config(config);
    }
}
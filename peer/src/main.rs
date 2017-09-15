#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate crypto;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;
extern crate serde_yaml;
extern crate simplelog;
extern crate time;
extern crate uuid;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_contrib;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod block;
mod blockchain;
mod config;
mod connections;
mod guards;
mod message;
mod peer;

use simplelog::{Config as SConfig, TermLogger, LogLevelFilter};
use rocket::config::{Config as RConfig, Environment};
use config::Config;

fn main() {
    let config = config::Config::new();

    //prepare_logger();
    message::register_at_peers(&config);

    rocket(&config).launch();

    println!("Peer ready.");
}

fn rocket(config: &Config) -> rocket::Rocket {
    let rocket_config = RConfig::build(Environment::Development)
        .address("0.0.0.0")
        .port(config.port)
        .finalize()
        .unwrap();

    rocket::custom(rocket_config, true)
        .manage(connections::postgres::init(&config.database))
        .mount("/api/block", routes![block::resources::new])
        .mount(
            "/api/blockchain",
            routes![
                blockchain::resources::new,
                blockchain::resources::overview,
            ],
        )
        .mount(
            "/api/peer",
            routes![peer::resources::list, peer::resources::register],
        )
}

fn prepare_logger() {
    TermLogger::init(LogLevelFilter::Info, SConfig::default())
        .expect("Could not initialize logger");
}
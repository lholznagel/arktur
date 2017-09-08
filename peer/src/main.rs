#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate crypto;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_yaml;
extern crate simplelog;
extern crate time;
extern crate uuid;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

mod api;
mod config;
mod connections;
mod guards;

use simplelog::{Config as SConfig, TermLogger, LogLevelFilter};
use rocket::config::{Config as RConfig, Environment};

fn main() {
    prepare_logger();

    rocket().launch();

    info!("Peer ready.");
}

fn rocket() -> rocket::Rocket {
    let config = config::Config::new();
    let rocket_config = RConfig::build(Environment::Development).port(config.port).finalize().unwrap();

    rocket::custom(rocket_config, true)
        .manage(connections::postgres::init(&config.database))
        .mount("/api/block", routes![api::block::resources::new])
        .mount(
            "/api/blockchain",
            routes![
                api::blockchain::resources::new,
                api::blockchain::resources::overview,
            ],
        )
        .mount(
            "/api/peer",
            routes![api::peer::resources::list, api::peer::resources::register],
        )
}

fn prepare_logger() {
    TermLogger::init(LogLevelFilter::Info, SConfig::default()).expect("Could not initialize logger");
}
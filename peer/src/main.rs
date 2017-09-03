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

mod block;
mod blockchain;
mod config;
mod connections;
mod guards;

use simplelog::{Config, TermLogger, LogLevelFilter};

fn main() {
    prepare_logger();

    rocket().launch();

    info!("Peer ready.");
}

fn rocket() -> rocket::Rocket {
    config::Config::new();

    rocket::ignite()
        .manage(connections::postgres::init())
        .mount("/api/block", routes![block::resources::new])
        .mount(
            "/api/blockchain",
            routes![blockchain::resources::new, blockchain::resources::overview],
        )
}

fn prepare_logger() {
    TermLogger::init(LogLevelFilter::Info, Config::default()).expect("Could not initialize logger");
}
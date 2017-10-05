extern crate iron;
extern crate router;
extern crate persistent;
extern crate plugin;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

mod config;
mod connection;
mod network;

use config::Config;
use connection::{init_database, Database};
use iron::prelude::{Chain, Iron};
use network::mount::{get_peers, register_peer};
use persistent::Read;
use router::Router;

fn main() {
    let config = Config::load();

    let pool = init_database(&config.database);

    let mut router = Router::new();
    router.get("/peers", get_peers, "")
        .post("/peers", register_peer, "");

    let mut chain = Chain::new(router);
    chain.link(Read::<Database>::both(pool));

    Iron::new(chain).http(format!("{}:{}", config.info.address, config.port)).unwrap();
}
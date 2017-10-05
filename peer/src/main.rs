extern crate iron;
extern crate mount;
extern crate persistent;
extern crate plugin;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

mod config;
mod network;
mod connection;

use connection::{init_database, Database};
use config::Config;
use iron::prelude::{Chain, Iron};
use mount::Mount;
use network::mount::foo;
use persistent::Read;

fn main() {
    let config = Config::load();

    let pool = init_database(&config.database);

    let mut mount = Mount::new();
    mount.mount("/foo", foo);

    let mut chain = Chain::new(mount);
    chain.link(Read::<Database>::both(pool));

    Iron::new(chain).http(format!("{}:{}", config.info.address, config.port)).unwrap();
}
extern crate crypto;
extern crate futures;
extern crate hyper;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde_yaml;
extern crate time;
extern crate tokio_core;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod config;
mod connections;
mod message;
mod peer;
mod server;

use connections::Pool;
use hyper::server::Http;
use server::PeerService;
use std::thread;

fn main() {
    let config = config::Config::new();

    let postgres = connections::postgres::init(&config.database);
    let server = thread::spawn(move || start_server(postgres));
    message::register_at_peers(&config);

    // get rocket back into the foreground
    println!("Peer ready.");
    server.join().unwrap();
}
fn start_server(postgres: Pool) {
    let config = config::Config::new();

    let mut url = String::from("0.0.0.0:");
    url.push_str(config.port.to_string().as_str());

    let address = url.parse().unwrap();
    let server = Http::new().bind(&address, move || Ok(PeerService::new(postgres.clone()))).unwrap();
    server.run().unwrap();
}
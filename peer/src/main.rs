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

//mod guards;
mod config;
mod connections;
mod message;
mod peer;
mod server;

//use rocket::config::{Config as RConfig, Environment};
use config::Config;
use connections::Pool;
use hyper::server::Http;
use server::PeerService;

fn main() {
    let config = config::Config::new();
    //let rocket_config = config.clone();

    let postgres = connections::postgres::init(&config.database);
    start_server(postgres);
    // set rocket into background so that we can register the peer
    //let rocket = thread::spawn(move || rocket(rocket_config).launch());
    message::register_at_peers(&config);

    // get rocket back into the foreground
    //rocket.join().unwrap();

    println!("Peer ready.");
}

/*fn rocket(config: Config) -> rocket::Rocket {
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
}*/

fn start_server(postgres: Pool) {
    let config = config::Config::new();

    let mut url = String::from("0.0.0.0:");
    url.push_str(config.port.to_string().as_str());

    let address = url.parse().unwrap();
    let server = Http::new().bind(&address, move || Ok(PeerService::new(postgres.clone()))).unwrap();
    server.run().unwrap();
}
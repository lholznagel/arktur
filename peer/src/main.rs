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

extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod api;
mod config;
mod connections;
mod guards;

use simplelog::{Config as SConfig, TermLogger, LogLevelFilter};
use rocket::config::{Config as RConfig, Environment};

use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType};
use tokio_core::reactor::Core;

fn main() {
    prepare_logger();
    test_register();

    rocket().launch();

    info!("Peer ready.");
}

fn rocket() -> rocket::Rocket {
    let config = config::Config::new();
    let rocket_config = RConfig::build(Environment::Development)
        .port(config.port)
        .finalize()
        .unwrap();

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
    TermLogger::init(LogLevelFilter::Info, SConfig::default())
        .expect("Could not initialize logger");
}

fn test_register() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = "http://localhost:8002/api/peer".parse().unwrap();

    let json = r#"{"name":"Peer1","id":"SomeId"}"#;
    println!("{:?}", json.len() as u64);
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());

        res.body().concat2()
    });

    core.run(post).unwrap();
}
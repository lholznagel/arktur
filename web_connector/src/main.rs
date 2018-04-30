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

//! RESTful API Server for communicating with the blockchain

extern crate actix;
extern crate actix_web;
extern crate base64;
extern crate carina_logging;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate sodiumoxide;

mod config;

use actix_web::{App, HttpRequest, HttpResponse, server};
use actix_web::http::StatusCode;
use carina_logging::LogBuilder;
use config::Config;
use std::fs::File;
use std::io::Read;

fn index(_: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).body("Hello world!")
}

fn main() {
    LogBuilder::new()
        .set_level(log::Level::Debug)
        .add_exclude("actix_web::server".to_string())
        .add_exclude("tokio_core".to_string())
        .add_exclude("tokio_reactor".to_string())
        .build()
        .unwrap();

    let mut file = File::open("./config.yml".to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();

    info!("Starting server");

    let sys = actix::System::new("carina");

    server::new(
        || App::new()
            .resource("/", |r| r.get().f(index)))
        .threads(4)
        .bind(format!("0.0.0.0:{}", config.port)).unwrap()
        .start();
    info!("Server ready");
    let _ = sys.run();
}
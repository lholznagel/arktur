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
extern crate carina_logging;
#[macro_use]
extern crate log;

use actix_web::{App, HttpRequest, HttpResponse, server};
use actix_web::http::StatusCode;
use carina_logging::LogBuilder;

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
    info!("Starting server");

    let sys = actix::System::new("carina");

    server::new(
        || App::new()
            .resource("/", |r| r.get().f(index)))
        .threads(4)
        .bind("127.0.0.1:8080").unwrap()
        .start();

    debug!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
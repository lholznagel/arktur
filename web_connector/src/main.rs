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
extern crate futures;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate sodiumoxide;
extern crate uuid;

mod config;
mod db;

use actix_web::{App, AsyncResponder, Error, http, HttpRequest, HttpResponse, server};
use actix::{Addr, Syn};
use actix::sync::SyncArbiter;
use carina_logging::LogBuilder;
use config::Config;
use db::{InsertBlock, DbExecutor};
use futures::future::Future;
use r2d2_sqlite::SqliteConnectionManager;
use std::fs::File;
use std::io::Read;

struct State {
    db: Addr<Syn, DbExecutor>,
}

fn index(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.state().db.send(InsertBlock {
            index: 0,
            timestamp: 0,
            nonce: 0,
            prev: String::new(),
            hash: String::new(),
            content: String::new()
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

fn main() {
    LogBuilder::new()
        .set_level(log::Level::Debug)
        .add_exclude("actix_web::server".to_string())
        .add_exclude("tokio_core".to_string())
        .add_exclude("tokio_reactor".to_string())
        .build()
        .unwrap();

    let sys = actix::System::new("carina");
    let manager = SqliteConnectionManager::file("test.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    let mut file = File::open("./config.yml".to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();

    info!("Starting server");

    server::new(move || {
        App::with_state(State{db: addr.clone()})
            .resource("/", |r| r.method(http::Method::GET).a(index))})
        .bind(format!("0.0.0.0:{}", config.port)).unwrap()
        .start();

    info!("Server ready");
    let _ = sys.run();
}
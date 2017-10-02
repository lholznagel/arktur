extern crate iron;
extern crate mount;
extern crate persistent;
extern crate plugin;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

mod network;
mod connection;

use connection::{Database, PostgresPool};
use iron::prelude::{Chain, Iron, Request, Response, IronResult};
use iron::status;
use mount::Mount;
use network::mount::foo;
use persistent::Read;
use r2d2_postgres::PostgresConnectionManager;

fn setup_connection_pool() -> PostgresPool {
    let manager = PostgresConnectionManager::new(
        "postgres://peer1:peer1@postgres:5432",
        ::r2d2_postgres::TlsMode::None,
    ).unwrap();
    let config = ::r2d2::Config::builder().pool_size(6).build();
    ::r2d2::Pool::new(config, manager).unwrap()
}

fn main() {
    let pool = setup_connection_pool();

    let mut mount = Mount::new();
    mount.mount("/foo", foo).mount("bar", bar);

    let mut chain = Chain::new(mount);
    chain.link(Read::<Database>::both(pool));

    Iron::new(chain).http("peer_1:8001").unwrap();
}

fn bar(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello bar")))
}
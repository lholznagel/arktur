extern crate iron;
extern crate mount;
extern crate persistent;
extern crate plugin;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

mod network;
mod connection;

use connection::{init_database, Database};
use iron::prelude::{Chain, Iron, Request, Response, IronResult};
use iron::status;
use mount::Mount;
use network::mount::foo;
use persistent::Read;

fn main() {
    let pool = init_database();

    let mut mount = Mount::new();
    mount.mount("/foo", foo).mount("bar", bar);

    let mut chain = Chain::new(mount);
    chain.link(Read::<Database>::both(pool));

    Iron::new(chain).http("peer_1:8001").unwrap();
}

fn bar(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello bar")))
}
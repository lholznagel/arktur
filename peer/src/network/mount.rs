use connection::Database;
use iron::prelude::{Request, Response, IronResult};
use iron::status;
use network::NetworkService;
use persistent::Read;
use plugin::Pluggable;
use serde_json::to_string;

pub fn get_network(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<Database>>().unwrap();
    let service = NetworkService::new(pool.get().unwrap());
    Ok(Response::with((status::Ok, to_string(&service.get_nodes()).unwrap())))
}

pub fn register_node(req: &mut Request) -> IronResult<Response> {
     Ok(Response::with((status::Ok, "Ok")))
}
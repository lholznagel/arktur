use iron::prelude::{Request, Response, IronResult};
use iron::status;
use persistent::Read;
use plugin::Pluggable;
use connection::{Database, PostgresPool};

pub fn foo(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<Database>>().unwrap();
    pool.get().unwrap().query("SELECT * FROM peers", &[]).unwrap();
    Ok(Response::with((status::Ok, "Hello foo")))
}
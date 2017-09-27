use connections::Pool;
use futures::future::{FutureResult, ok};
use futures::{BoxFuture, Future, Stream};
use hyper::server::{Service, Request, Response};
use hyper::{Chunk, Error, Get, Post, StatusCode};
use hyper::header::ContentLength;
use message::Message;
use peer::Register;
use peer::service::{get_all_peers, save_peer};
use serde_json::from_str;
use config::Config;
use connections::postgres;

pub struct PeerService {
    pool: Pool,
}

impl PeerService {
    pub fn new(pool: Pool) -> Self {
        PeerService { pool: pool }
    }
}

impl Service for PeerService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = BoxFuture<Response, Error>;

    fn call(&self, req: Request) -> Self::Future {
        let config = Config::new();
        let postgres = postgres::init(&config.database);
        //let (method, uri, _version, headers, body) = req.deconstruct();
        match (req.method(), req.path()) {
            /*(&Get, "/api/peer") => {
                Response::new().with_status(StatusCode::Ok).with_body(
                    get_all_peers(
                        &self.pool,
                    ),
                )
            }*/
            (&Post, "/api/peer") => {
                let mut res = Response::new();

                req.body()
                    .fold(Vec::new(), |mut value, chunk| {
                        value.extend(&chunk[..]);
                        Ok::<_, Error>(value)
                    })
                    .and_then(move |chunks| {
                        let body = String::from_utf8(chunks).unwrap();
                        let converted: Message<Register> = from_str(&body).unwrap();
                        save_peer(postgres, &converted);
                        println!("{:?}", converted);
                        Ok(res.with_status(StatusCode::Ok))
                    })
                    .boxed()
            }
            _ => ok(Response::new().with_status(StatusCode::NotFound)).boxed(),
        }
    }
}
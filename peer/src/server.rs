use connections::Pool;
use hyper::{Error, Get, Post, StatusCode};
use hyper::server::{Service, Request, Response};
use futures::future::{FutureResult, ok};
use peer::service::{get_all_peers, save_peer};
use peer::Register;
use message::Message;
use serde_json::from_str;

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
    type Future = FutureResult<Response, Error>;

    fn call(&self, request: Request) -> Self::Future {
        ok(match (request.method(), request.path()) {
            (&Get, "/api/peer") => {
                Response::new().with_status(StatusCode::Ok).with_body(
                    get_all_peers(
                        &self.pool,
                    ),
                )
            },
            (&Post, "/api/peer") => {
                let body: Message<Register> = from_str(request.body());

                Response::new().with_status(StatusCode::Ok).with_body(
                    save_peer(
                        &self.pool,
                        body
                    ),
                )
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}
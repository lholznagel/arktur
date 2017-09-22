use connections::Pool;
use hyper::{Chunk, Error, Get, Post, StatusCode};
use hyper::server::{Service, Request, Response};
use futures::future::{FutureResult, ok};
use peer::service::{get_all_peers, save_peer};
use peer::Register;
use message::Message;
use serde_json::{Value, from_slice};
use futures::{Future, Stream};

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
                let response = Response::new().with_status(StatusCode::Ok);
                request.body().concat2().and_then(move | body: Chunk | {
                    let value: Message<Register> = from_slice(&body).unwrap();
                    save_peer(&self.pool, &value);
                    Ok({})
                });

                response
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}
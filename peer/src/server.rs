use connections::Pool;
use futures::future::{FutureResult, ok};
use futures::{BoxFuture, Future, Stream};
use hyper::server::{Service, Request, Response};
use hyper::{Chunk, Error, Get, Post, StatusCode};
use hyper::header::ContentLength;
use message::Message;
use peer::Register;
use peer::service::{get_all_peers, save_peer};
use serde_json::from_slice;

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
        let (method, uri, _version, headers, body) = req.deconstruct();
        match (method, uri.path()) {
            (Post, "/api/peer") => {
                let mut res = Response::new();
                let vec;
                if let Some(len) = headers.get::<ContentLength>() {
                    vec = Vec::with_capacity(**len as usize);
                    res.headers_mut().set(len.clone());
                } else {
                    vec = vec![];
                }
                body.fold(vec, |mut acc, chunk| {
                    println!("{:?}", chunk);
                    acc.extend_from_slice(chunk.as_ref());
                    Ok::<_, Error>(acc)
                }).and_then(move |value| {
                    //println!("value: {:?}", &value);
                    Ok(res.with_body(value))
                }).boxed()
            },
            _ => ok(Response::new().with_status(StatusCode::NotFound)).boxed()
        }
    }
}

/*impl Service for PeerService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = FutureResult<Response, Error>;

    fn call(&self, request: Request) -> Self::Future {
        let (method, uri, _version, _headers, body) = request.deconstruct();

        ok(match (&method, uri.path()) {
            (&Get, "/api/peer") => {
                Response::new().with_status(StatusCode::Ok).with_body(
                    get_all_peers(
                        &self.pool,
                    ),
                )
            }
            (&Post, "/api/peer") => {
                println!("/api/peer post");
                let response = Response::new().with_status(StatusCode::Ok);
                /*request.body().concat2().and_then(|body: Chunk| {
                    let value: Message<Register> = from_slice(&body).unwrap();
                    println!("asdads");
                    save_peer(&self.pool, &value);
                    Ok({})
                }).wait();*/

                /*body.concat2().and_then(|result| {
                    println!("{:?} h", result);
                    Ok({})
                });*/
                let v = body.collect().wait().unwrap();
                println!("{:?}", v);

                response
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}*/
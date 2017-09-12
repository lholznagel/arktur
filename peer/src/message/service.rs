use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType};
use tokio_core::reactor::Core;
use uuid::Uuid;
use config::Config;
use peer::{Message, Messagable, Register};

pub fn register_at_peers(config: &Config) {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = "http://localhost:8002/api/peer".parse().unwrap();

    let json = Message {
        content: Register {
            name: String::from("Test"),
            address: String::from("localhost"),
            port: 8001,
        },
        id: Uuid::new_v4(),
        timestamp: 0,
        hash: String::from("asd"),
        is_valid_hash: true,
    }.as_json().to_string();

    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());

        res.body().concat2()
    });

    match core.run(post) {
        Ok(_) => {},
        Err(_) => println!("Could not reach host"),
    };
}
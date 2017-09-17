use config::Config;
use futures::{Future, Stream};
use hyper::header::{ContentLength, ContentType};
use hyper::{Client, Method, Request, Uri};
use message::{Messagable, Message};
use peer::Register;
use time::get_time;
use tokio_core::reactor::Core;
use uuid::Uuid;
use std::{thread, time};


pub fn register_at_peers(config: &Config) {
    let message_id = Uuid::new_v4();
    let peer_id = Uuid::new_v4();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    for peer in &config.peers {
        loop {
            let json = Message {
                content: Register {
                    name: config.info.name.clone(),
                    address: config.info.address.clone(),
                    port: config.port as i32,
                    peer_id: peer_id,
                    notify_on_change: true,
                },
                id: message_id,
                timestamp: get_time().sec,
                hash: String::from(""),
                is_valid_hash: false,
            };
            let json = json.generate_hash().as_json().to_string();

            let mut req = Request::new(Method::Post, build_peer_uri(&peer.address, &peer.port));
            req.headers_mut().set(ContentType::json());
            req.headers_mut().set(ContentLength(json.len() as u64));
            req.set_body(json);

            let post = client.request(req).and_then(|res| res.body().concat2());

            match core.run(post) {
                Ok(_) => break,
                Err(error) => {
                    println!("{:?}", error);
                    println!("Error during register.");
                    thread::sleep(time::Duration::from_secs(5));
                }
            }
        }
    }
}

/*pub fn register_at_peers(config: &Config) {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let message_id = Uuid::new_v4();
    let peer_id = Uuid::new_v4();

    for peer in &config.peers {
        let json = Message {
            content: Register {
                name: config.info.name.clone(),
                address: config.info.address.clone(),
                port: config.port as i32,
                peer_id: peer_id,
                notify_on_change: true,
            },
            id: message_id,
            timestamp: get_time().sec,
            hash: String::from(""),
            is_valid_hash: false,
        };
        let json = json.generate_hash().as_json().to_string();

        let mut req = Request::new(Method::Post, build_peer_uri(&config, &peer.port));
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = client.request(req).and_then(|res| res.body().concat2());

        match core.run(post) {
            Ok(_) => {}
            Err(_) => println!("Error during registration."),
        };
    }
}*/

fn build_peer_uri(peer_address: &String, peer_port: &u16) -> Uri {
    let mut connection_string = String::from("http://");
    connection_string.push_str(peer_address.as_str());
    connection_string.push_str(":");
    connection_string.push_str(peer_port.to_string().as_str());
    connection_string.push_str("/api/peer");
    println!("{:?}", connection_string);
    connection_string.parse().unwrap()
}
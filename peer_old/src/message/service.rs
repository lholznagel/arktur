use config::Config;
use futures::{Future, Stream};
//use guards::DBConnection;
use hyper::header::{ContentLength, ContentType};
use hyper::{Client, Method, Request, Uri};
use message::{Messagable, Message};
use peer::Register;
use std::{thread, time};
use time::get_time;
use tokio_core::reactor::Core;
use uuid::Uuid;
use connections::Pool;

struct PeerToNotify {
    address: String,
    port: i32,
}

pub fn notify_new_peer(pool: &Pool, message: &Message<Register>) {
    println!("notify_new_peer START");
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    for row in &pool.get().unwrap()
        .query(
            "SELECT address, port FROM peers WHERE notify_on_change = true",
            &[],
        )
        .unwrap()
    {
        loop {
            println!("notify_new_peer LOOP START");
            let peer = PeerToNotify {
                address: row.get(0),
                port: row.get(1),
            };

            let json = message.as_json().to_string();
            println!("notify_new_peer JSON {:?}", json);

            let mut req = Request::new(Method::Post, build_peer_uri(&peer.address, &peer.port));
            req.headers_mut().set(ContentType::json());
            req.headers_mut().set(ContentLength(json.len() as u64));
            req.set_body(json);

            let post = client.request(req).and_then(|res| {
                println!("notify_new_peer EXEC");
                res.body().concat2()
            });

            match core.run(post) {
                Ok(_) => {
                    println!("notify_new_peer MESSAGE SUCCESS");
                    break
                },
                Err(error) => {
                    println!("{:?}", error);
                    // TODO problem: the peer wont answer, possible timeout
                    println!("Error notifying peer. Waiting 1 seconds.");
                    thread::sleep(time::Duration::from_secs(1));
                }
            }

            println!("notify_new_peer LOOP END");
        }
    }
    println!("notify_new_peer DONE");
}

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
                    println!("Error during register. Waiting 2 seconds.");
                    thread::sleep(time::Duration::from_secs(2));
                }
            }
        }
    }
}

fn build_peer_uri(peer_address: &String, peer_port: &i32) -> Uri {
    let mut connection_string = String::from("http://");
    connection_string.push_str(peer_address.as_str());
    connection_string.push_str(":");
    connection_string.push_str(peer_port.to_string().as_str());
    connection_string.push_str("/api/peer");
    connection_string.parse().unwrap()
}
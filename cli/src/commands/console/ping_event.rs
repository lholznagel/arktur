use carina_core::Event;
use carina_core::Config;
use carina_core_protocol::{MessageBuilder, Payload};
use carina_core_protocol::payloads::EmptyPayload;
use std::net::UdpSocket;

pub struct Ping;

impl Event for Ping {
    fn execute(&mut self, udp: UdpSocket, source: String, config: &mut Config) {
        info!("[CONSOLE_PING] Received ping event from {:?}", source);
        let message = MessageBuilder::new()
            .set_event_code(1)
            .set_payload(EmptyPayload::new())
            .build(&mut config.nacl, &config.peers.get(&source).unwrap().public_key);

        match udp.send_to(&message, &source) {
            Ok(_)  => debug!("[CONSOLE_PING] Sending pong to peer {}", source),
            Err(e) => error!("[CONSOLE_PING] Error sending pong to peer: {}. Error: {}", source, e),
        };
    }
}
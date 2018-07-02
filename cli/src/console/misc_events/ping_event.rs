use carina_core_protocol::{Events, MessageBuilder, Payload};
use carina_core_protocol::payloads::EmptyPayload;
use carina_core::Config;
use carina_core::Event;
use failure::Error;
use std::net::UdpSocket;

pub struct Ping;

impl Event for Ping {
    fn execute(&mut self, udp: UdpSocket, source: String, config: &mut Config, _: &[u8]) -> Result<(), Error> {
        info!("[CONSOLE_PING] Received ping event from {:?}", source);
        match config.peers.get(&source) {
            Some(peer) => {
                let message = MessageBuilder::new()
                    .set_event_code(Events::as_val(Events::Pong))
                    .set_payload(EmptyPayload::new())
                    .build(&mut config.nacl, &peer.public_key);

                match udp.send_to(&message, &source) {
                    Ok(_)  => debug!("[CONSOLE_PING] Sending pong to peer {}", source),
                    Err(e) => error!("[CONSOLE_PING] Error sending pong to peer: {}. Error: {}", source, e),
                };
            },
            None => error!("[CONSOLE_PING] Error getting peer")
        };

        Ok(())
    }
}
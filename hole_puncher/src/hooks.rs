use blockchain_hooks::{EventCodes, Hooks};
use blockchain_protocol::payload::{Payload, RegisterPayload, RegisterAckPayload};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;

use std::net::UdpSocket;
use std::collections::HashMap;

pub struct HookHandler {
    peers: HashMap<String, String>
}

impl HookHandler {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new()
        }
    }
}

impl Hooks for HookHandler {
    fn on_register_hole_puncher(&mut self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<RegisterPayload>::from_bytes(&payload_buffer).expect("Parsing payload should be successful.");

        if self.peers.is_empty() {
            sending!("ACK_REGISTER | NO_PEER");
            let answer = BlockchainProtocol::new()
                .set_event_code(EventCodes::RegisterHolePuncherAck)
                .set_status_code(StatusCodes::NoPeer)
                .set_payload(RegisterAckPayload::new())
                .build();
            udp.send_to(&answer, source.clone()).expect("Sending a response should be successful");
        } else {
            sending!("ACK_REGISTER | PEER");
            let answer = BlockchainProtocol::new()
                .set_event_code(EventCodes::RegisterHolePuncherAck)
                .set_status_code(StatusCodes::Ok)
                .set_payload(RegisterAckPayload::new().set_peers(&self.peers))
                .build();
            udp.send_to(&answer, source.clone()).expect("Sending a response should be successful");
        }

        self.peers.insert(source, message.payload.name);
    }

    fn on_ping(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_pong(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_hole_puncher_ack(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer_ack(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_new_block(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_possible_block(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validate_hash(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validated_hash(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_found_block(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
}
use blockchain_file::blocks::Block;
use blockchain_hooks::{EventCodes, Hooks};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{Payload, FoundBlockPayload, PongPayload, RegisterPayload, RegisterAckPayload, NewBlockPayload, PossibleBlockPayload, ValidateHashPayload, ValidatedHashPayload};

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::collections::HashMap;
use std::net::UdpSocket;

/// Contains all hooks that the peer listens to
pub struct HookHandler {
    peers: HashMap<String, String>
}

impl HookHandler {
    /// Creates a new empty instance of HookHandler
    pub fn new() -> Self {
        Self {
            peers: HashMap::new()
        }
    }
}

impl Hooks for HookHandler {
    fn on_ping(&self, udp: &UdpSocket, _: Vec<u8>, source: String) {
        event!("PING from peer {:?}", source);
        sending!("PONG to peer {:?}", source);
        success!("Send PONG to {:?}", source);
        let answer = BlockchainProtocol::<PongPayload>::new().set_event_code(EventCodes::Pong).build();
        udp.send_to(&answer, source).expect("Sending a response should be successful");
    }

    fn on_pong(&self, _: &UdpSocket, _: Vec<u8>, source: String) {
        event!("PONG from peer {:?}", source);
     }

    fn on_register_hole_puncher_ack(&mut self, udp: &UdpSocket, payload_buffer: Vec<u8>, _: String) {
        let message = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        event!("ACK_REGISTER {:?}", message);

        if message.status_code == StatusCodes::NoPeer {
            error!("No peer");
        } else {
            sending!("PING to peer {:?}", message.payload);

            for address in message.payload.addresses {
                success!("Send REGISTER_PEER to {:?}", address);
                let result = BlockchainProtocol::<RegisterPayload>::new().set_event_code(EventCodes::RegisterPeer).build();
                udp.send_to(&result, address).expect("Sending a response should be successful");
            }
        }
     }

     fn on_register_peer(&mut self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<RegisterPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        event!("ACK_REGISTER {:?}", message);

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

    fn on_new_block(&self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<NewBlockPayload>::from_bytes(&payload_buffer).unwrap();
        event!("NEW_BLOCK {:?}", message.payload);
    
        let hash;
        let mut nonce = 0;

        loop {
            let mut generated_block = String::from("");
            generated_block.push_str(&message.payload.content);
            generated_block.push_str(&message.payload.index.to_string());
            generated_block.push_str(&message.payload.timestamp.to_string());
            generated_block.push_str(&message.payload.prev);
            generated_block.push_str(&nonce.to_string());

            let mut hasher = Sha3::sha3_256();
            hasher.input_str(generated_block.as_str());
            let hex = hasher.result_str();

            if message.payload.sign_key == &hex[..message.payload.sign_key.len()] {
                hash = hex.clone();
                break;
            } else {
                nonce += 1;
            }
        }

        debug!("Found hash! {:?}", hash);
        let mut answer = BlockchainProtocol::<PossibleBlockPayload>::new().set_event_code(EventCodes::PossibleBlock);
        answer.payload.content = message.payload.content;
        answer.payload.timestamp = message.payload.timestamp;
        answer.payload.index = message.payload.index;
        answer.payload.prev = message.payload.prev;
        answer.payload.nonce = nonce;
        answer.payload.hash = hash;
        sending!("POSSIBLE_BLOCK | {:?}", answer.payload);
        success!("Send block back.");
        udp.send_to(&answer.build(), source).expect("Sending a response should be successful");
    }

    fn on_validate_hash(&self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<ValidateHashPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        event!("VALIDATE_HASH {:?}", message.payload);

        let mut generated_block = String::from("");
        generated_block.push_str(&message.payload.content);
        generated_block.push_str(&message.payload.index.to_string());
        generated_block.push_str(&message.payload.timestamp.to_string());
        generated_block.push_str(&message.payload.prev);
        generated_block.push_str(&message.payload.nonce.to_string());

        let mut hasher = Sha3::sha3_256();
        hasher.input_str(generated_block.as_str());

        let mut answer = BlockchainProtocol::<ValidatedHashPayload>::new().set_event_code(EventCodes::ValidatedHash);
        answer.payload.index = message.payload.index;
        answer.payload.hash = hasher.result_str();
        udp.send_to(&answer.build(), source).expect("Sending a response should be successful");
    }

    fn on_found_block(&self, _: &UdpSocket, payload_buffer: Vec<u8>, _: String) {
        let message = BlockchainProtocol::<FoundBlockPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        event!("FOUND_BLOCK {:?}", message.payload);

        Block::init();
        let mut block = Block::new();
        block.index = message.payload.index;
        block.content = message.payload.content;
        block.timestamp = message.payload.timestamp;
        block.nonce = message.payload.nonce;
        block.prev = message.payload.prev;
        block.hash = message.payload.hash;
        block.save();
    }

    fn on_register_hole_puncher(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer_ack(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_possible_block(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validated_hash(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
}
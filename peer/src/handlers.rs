use blockchain_file::blocks::Block;
use blockchain_hooks::{EventCodes, Hooks};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{FoundBlockPayload, PingPayload, PongPayload, RegisterAckPayload, PeerRegisteringPayload, NewBlockPayload, PossibleBlockPayload, ValidateHashPayload, ValidatedHashPayload};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::net::UdpSocket;

/// Contains all hooks that the peer listens to
pub struct HookHandlers;

impl Hooks for HookHandlers {
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

    fn on_ack_register(&self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        event!("ACK_REGISTER {:?}", message);

        if message.status_code == StatusCodes::NoPeer {
            error!("No peer");
        } else {
            sending!("PING to peer {:?}", message.payload);
            success!("Send PING to {:?}", source);
            let result = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
            udp.send_to(&result, source).expect("Sending a response should be successful");
        }
     }

    fn on_peer_registering(&self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<PeerRegisteringPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();

        event!("PEER_REGISTERING {:?}", message.payload);
        sending!("PING to new peer {:?}", message.payload);
        success!("Send PING {:?}", message.payload);
        let answer = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
        udp.send_to( &answer, source).expect("Sending a response should be successful");
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

    fn on_register(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_possible_block(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validated_hash(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
}
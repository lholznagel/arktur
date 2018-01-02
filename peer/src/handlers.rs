use blockchain_hooks::{EventCodes, Hooks};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{FoundBlockPayload, PingPayload, PongPayload, RegisterAckPayload, PeerRegisteringPayload, NewBlockPayload, PossibleBlockPayload, ValidateHash, ValidatedHash};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::net::UdpSocket;

/// Contains all hooks that the peer listens to
pub struct HookHandlers;

impl Hooks for HookHandlers {
    fn on_ping(&self, _: Vec<u8>, source: String) -> Vec<u8> { 
        event!(format!("PING from peer {:?}", source));
        sending!(format!("PONG to peer {:?}", source));
        let answer = BlockchainProtocol::<PongPayload>::new().set_event_code(EventCodes::Pong).build();
        success!(format!("Send PONG to {:?}", source));
        answer
    }

    fn on_pong(&self, _: Vec<u8>, source: String) -> Vec<u8> { 
        event!(format!("PONG from peer {:?}", source));
        Vec::new()
     }

    fn on_ack_register(&self, payload_buffer: Vec<u8>, _: String) -> Vec<u8> { 
        let mut result = Vec::new();
        let message = BlockchainProtocol::<RegisterAckPayload>::from_vec(payload_buffer);
        event!(format!("ACK_REGISTER {:?}", message));

        if message.status_code == StatusCodes::NoPeer {
            error!("No peer");
        } else {
            sending!(format!("PING to peer {:?}", message.payload));
            result = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
            success!(format!("Send PING to {:?}", message.payload));
        }
        result
     }

    fn on_peer_registering(&self, payload_buffer: Vec<u8>, _: String) -> Vec<u8> { 
        let message = BlockchainProtocol::<PeerRegisteringPayload>::from_vec(payload_buffer);

        event!(format!("PEER_REGISTERING {:?}", message.payload));
        sending!(format!("PING to new peer {:?}", message.payload));
        let answer = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
        success!(format!("Send PING {:?}", message.payload));
        answer
     }

    fn on_new_block(&self, payload_buffer: Vec<u8>, _: String) -> Vec<u8> { 
        let message = BlockchainProtocol::<NewBlockPayload>::from_vec(payload_buffer);
        event!(format!("NEW_BLOCK {:?}", message.payload));
    
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

        debug!(format!("Found hash! {:?}", hash));
        let mut answer = BlockchainProtocol::<PossibleBlockPayload>::new().set_event_code(EventCodes::PossibleBlock);
        answer.payload.content = message.payload.content;
        answer.payload.timestamp = message.payload.timestamp;
        answer.payload.index = message.payload.index;
        answer.payload.prev = message.payload.prev;
        answer.payload.nonce = nonce;
        answer.payload.hash = hash;
        sending!(format!("POSSIBLE_BLOCK | {:?}", answer.payload));
        success!("Send block back.");
        answer.build()
    }

    fn on_validate_hash(&self, payload_buffer: Vec<u8>, _: String) -> Vec<u8> { 
        let message = BlockchainProtocol::<ValidateHash>::from_vec(payload_buffer);
        event!(format!("VALIDATE_HASH {:?}", message.payload));

        let mut generated_block = String::from("");
        generated_block.push_str(&message.payload.content);
        generated_block.push_str(&message.payload.index.to_string());
        generated_block.push_str(&message.payload.timestamp.to_string());
        generated_block.push_str(&message.payload.prev);
        generated_block.push_str(&message.payload.nonce.to_string());

        let mut hasher = Sha3::sha3_256();
        hasher.input_str(generated_block.as_str());

        let mut answer = BlockchainProtocol::<ValidatedHash>::new().set_event_code(EventCodes::ValidatedHash);
        answer.payload.index = message.payload.index;
        answer.payload.hash = hasher.result_str();
        answer.build()
    }

    fn on_found_block(&self, payload_buffer: Vec<u8>, _: String) -> Vec<u8> { 
        let message = BlockchainProtocol::<FoundBlockPayload>::from_vec(payload_buffer);
        event!(format!("FOUND_BLOCK {:?}", message.payload));

        Vec::new()
    }

    fn on_register(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_possible_block(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_validated_hash(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
}
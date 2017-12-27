use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{PingPayload, PongPayload, RegisterAckPayload, NewBlockPayload, PeerRegisteringPayload, PossibleBlockPayload};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::net::{SocketAddr, UdpSocket};

use blockchain_network::event::{PingEvent, PongEvent};

/// Contains all event handlers
#[derive(Clone)]
pub struct EventHandlers;

impl EventHandlers {
    /// Creates a new instance of event handlers
    /// Only needed initially
    pub fn new() -> Self {
        EventHandlers
    }
}

impl PingEvent for EventHandlers {
    fn handle_event(self: Box<Self>, _: BlockchainProtocol<PingPayload>, source: SocketAddr) -> Vec<u8> {
        println!("PING EXEC");
        event!(format!("PING from peer {:?}", source.to_string()));
        sending!(format!("PONG to peer {:?}", source.to_string()));
        let answer = BlockchainProtocol::<PongPayload>::new().set_event_code(EventCodes::Pong).build();
        success!(format!("Send PONG to {:?}", source.to_string()));
        answer
    }
}

impl PongEvent for EventHandlers {
    fn handle_event(self: Box<Self>, _: BlockchainProtocol<PongPayload>, source: SocketAddr) -> Vec<u8> {
        event!(format!("PONG from peer {:?}", source.to_string()));
        vec![0]
    }
}

/// Event send when the connection manager acknowledge the register event
/// Sends a PING event if there are already connected peers
pub fn register_ack_handler(_: SocketAddr, udp: &UdpSocket, message: BlockchainProtocol<RegisterAckPayload>) {
     if message.status_code == StatusCodes::NoPeer {
         error!("No peer");
     } else {
        sending!(format!("PING to peer {:?}", message.payload));
        let answer = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
        udp.send_to(answer.as_slice(), message.payload.addr.parse::<SocketAddr>().unwrap()).unwrap();
        success!(format!("Send PING to {:?}", message.payload));
     }
}

/// Listens to new peers that register them self
/// Sends the new peer a PING event
pub fn peer_registering_handler(_: SocketAddr, udp: &UdpSocket, message: BlockchainProtocol<PeerRegisteringPayload>) {
    event!(format!("PEER_REGISTERING {:?}", message.payload));
    sending!(format!("PING to new peer {:?}", message.payload));
    let answer = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
    udp.send_to(answer.as_slice(), message.payload.addr.parse::<SocketAddr>().unwrap()).unwrap();
    success!(format!("Send PING {:?}", message.payload));
}

/// Listens when a new block should be calculated
/// For now the sign key is 0000. This should change in the future.
pub fn new_block_handler(source: SocketAddr, udp: &UdpSocket, message: BlockchainProtocol<NewBlockPayload>) {
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
    udp.send_to(answer.build().as_slice(), source).unwrap();
    success!("Send block back.");
}
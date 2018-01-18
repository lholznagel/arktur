use blockchain_file::peers::{KnownPeers, Peer};
use blockchain_hooks::Hooks;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{FoundBlockPayload, NewBlockPayload, Payload, RegisterAckPayload, PossibleBlockPayload, RegisterPayload, PeerRegisteringPayload, ValidateHashPayload, ValidatedHashPayload};
use blockchain_hooks::EventCodes;
use blockchain_protocol::BlockchainProtocol;

use std::net::{UdpSocket, SocketAddr};
use std::collections::HashMap;
use time::get_time;

pub struct HookHandlers {
    connected_peers_addr: Vec<String>,
    current_block: PossibleBlockPayload,
    hashes: Vec<String>,
    validation_in_progress: bool,
    last_block_time: i64
}

impl HookHandlers {
    pub fn new() -> Self {
        Self {
            connected_peers_addr: Vec::new(),
            current_block: PossibleBlockPayload::new(),
            hashes: Vec::new(),
            validation_in_progress: false,
            last_block_time: 0
        }
    }

    fn send_genesis(&mut self, udp: &UdpSocket) {
        let payload = NewBlockPayload::block(0, String::from("0".repeat(64)));
        self.last_block_time = payload.timestamp;
        self.validation_in_progress = false;

        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::NewBlock)
            .set_payload(payload)
            .build();

        for peer in self.connected_peers_addr.clone() {
            udp.send_to(
                message.as_slice(),
                peer.parse::<SocketAddr>().unwrap(),
            ).unwrap();
        }
    }

    fn send_next_block(&mut self, udp: &UdpSocket) {
        let payload = NewBlockPayload::block(self.current_block.index + 1, self.current_block.hash.clone());
        self.last_block_time = payload.timestamp;
        self.validation_in_progress = false;

        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::NewBlock)
            .set_payload(payload)
            .build();

        for peer in self.connected_peers_addr.clone() {
            udp.send_to(
                message.as_slice(),
                peer.parse::<SocketAddr>().unwrap(),
            ).unwrap();
        }
    }
}

impl Hooks for HookHandlers {
    /// # Hole puncher
    ///
    /// - Create a "hole" between to peers
    /// - When a peer registers itself, its IP-Address + Port are saved
    /// - The next peer that registers itself, gets these IP-Address + Port
    /// - The older peer gets the IP-Address + Port of the new peer
    /// - The address of the new peer are saved instead of the old peer
    /// - Both start a ping event to the other peer
    /// - With this technic a connection between two private networks can be accomplished
    ///
    /// In the following graphic, the process is shown
    ///
    /// ```
    ///  1. Register  +--------------+ 2. Register
    ///   +--------->|              |<---------+
    ///   |          | hole puncher |          |
    ///   |    +-----|              |-----+    |
    ///   |    |     +--------------+     |    |
    ///   |    | 3. Send IP+Port of new   |    |
    ///   |    |                          |    |
    ///   |    |                          |    |
    ///   |    |                          |    |
    ///   |    |   4. Send IP+Port of old |    |
    ///   |    v                          v    |
    /// +--------+                      +--------+
    /// |        |--------------------->|        |
    /// | Peer A |      5. Contact      | Peer B |
    /// |        |<---------------------|        |
    /// +--------+                      +--------+
    ///
    /// created with http://asciiflow.com/
    /// ```
    ///
    /// # Example
    ///
    /// - Peer A runs on 192.168.1.5:45678 (on host a)
    /// - Peer B runs on 192.168.1.6:56789 (on host b)
    /// - Peer A registers itself at the hole puncher (some.public.ip.address:45000)
    /// - The hole puncher does not know any peer
    /// - Peer B registers itself at the same hole puncher
    /// - The hole puncher sends the Peer B information to Peer A
    /// - The hole puncher then sends the Peer A information to Peer B
    /// - Peer A and Peer B try to ping each other
    /// - The connection between both networks should be good to go
    ///
    /// Handles a new peer
    fn on_register_hole_puncher(&mut self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<RegisterPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        
        let last_peer = KnownPeers::get_latest();
        let mut status = StatusCodes::Ok;

        if last_peer.get_name() == "" {
            status = StatusCodes::NoPeer;
        } else {
            let mut payload = PeerRegisteringPayload::new();
            payload.addr = source.to_string();

            /*let message = BlockchainProtocol::new()
                .set_event_code(EventCodes::PeerRegistering)
                .set_payload(payload)
                .build();
            udp.send_to(message.as_slice(), last_peer.get_socket().parse::<SocketAddr>().unwrap()).unwrap();*/
        }

        KnownPeers::new(Peer::new(message.payload.name, source.to_string())).save();
        self.connected_peers_addr.push(source.to_string());

        if self.connected_peers_addr.len() >= 3 && self.current_block.index == 0 {
            self.send_genesis(&udp);
        }

        let mut payload = RegisterAckPayload::new();
        payload.addr = String::from(last_peer.get_socket());

        sending!("ACK_REGISTER | {:?}", payload);
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterHolePuncherAck)
            .set_status_code(status)
            .set_payload(payload)
            .build();
        udp.send_to(&answer, source).expect("Sending a response should be successful");
    }

    fn on_possible_block(&mut self, udp: &UdpSocket, payload_buffer: Vec<u8>, _: String) {
        let message = BlockchainProtocol::<PossibleBlockPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        self.current_block = message.payload.clone();

        if self.current_block.index > message.payload.index {
            self.validation_in_progress = false;
        }

        event!("POSSIBLE_BLOCK | {:?}", message);

        if !self.validation_in_progress {
            let payload = ValidateHashPayload {
                content: message.payload.content,
                index: message.payload.index,
                nonce: message.payload.nonce,
                prev: message.payload.prev,
                timestamp: message.payload.timestamp
            };

            let message = BlockchainProtocol::new()
                .set_event_code(EventCodes::ValidateHash)
                .set_payload(payload)
                .build();

            for peer in self.connected_peers_addr.clone() {
                self.validation_in_progress = true;
                udp.send_to(message.as_slice(), peer.parse::<SocketAddr>().unwrap()).unwrap();
            }
        }
    }

    fn on_validated_hash(&mut self, udp: &UdpSocket, payload_buffer: Vec<u8>, _: String) {
        let message = BlockchainProtocol::<ValidatedHashPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        event!("VALIDATED_HASH | {:?}", message);

        if message.payload.index == self.current_block.index {
            self.hashes.push(message.payload.hash);
        }

        if self.hashes.len() == self.connected_peers_addr.len() {
            let mut hashes = HashMap::new();

            for hash in self.hashes.clone() {
                let updated_value = match hashes.get(&hash) {
                    Some(current_val)   => current_val + 1,
                    None                => 1
                };

                hashes.insert(hash, updated_value);
            }

            let mut result: (String, u64) = (String::from(""), 0);
            for (key, value) in hashes {
                if result.1 == 0 || value > result.1 {
                    result.0 = key;
                    result.1 = value;
                }
            }

            self.hashes = Vec::new();
            debug!("Hash {} for block: {:?}", result.0, self.current_block);

            self.current_block.hash = result.0;

            let mut payload = FoundBlockPayload::new();
            payload.content = self.current_block.content.clone();
            payload.index = self.current_block.index;
            payload.nonce = self.current_block.nonce;
            payload.prev = self.current_block.prev.clone();
            payload.timestamp = self.current_block.timestamp;
            payload.hash = self.current_block.hash.clone();

            let message = BlockchainProtocol::new()
                .set_event_code(EventCodes::FoundBlock)
                .set_payload(payload)
                .build();

            for peer in self.connected_peers_addr.clone() {
                udp.send_to(message.as_slice(), peer.parse::<SocketAddr>().unwrap()).unwrap();
            }

            loop {
                // for now every 2 minutes
                if get_time().sec - self.last_block_time >= 120 {
                    self.send_next_block(&udp);
                    break;
                }
            }
        }
    }

    fn on_ping(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_pong(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_hole_puncher_ack(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer_ack(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_new_block(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validate_hash(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_found_block(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
}

use carina_core_protocol::Events;
use carina_core_protocol::MessageBuilder;
use carina_core_protocol::Payload;
use carina_core_protocol::payloads::block::CalcBlockPayload;
use carina_core_protocol::payloads::EmptyPayload;
use carina_core::Config;
use carina_core::Event;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use failure::Error;
use protocol_builder_parser::Parser;
use std::net::UdpSocket;

pub struct CalcBlock {
    is_calculating: bool
}

impl CalcBlock {
    pub fn new() -> Self {
        Self {
            is_calculating: false
        }
    }
}

impl Event for CalcBlock {
    fn execute(&mut self, socket: UdpSocket, _: String, config: &mut Config, buffer: &[u8]) -> Result<(), Error> {
        let parsed = Parser::parse_payload(&buffer);
        let parsed = CalcBlockPayload::parse(parsed)?;

        let hash;
        let mut nonce = 0;

        let mut block_data = String::new();
        block_data.push_str(&parsed.index.to_string());
        block_data.push_str(&parsed.timestamp.to_string());
        block_data.push_str(&parsed.prev);
        block_data.push_str(&parsed.content);

        info!("[CONSOLE_CALC_BLOCK] Starting generating a new block.");
        self.is_calculating = true;
        loop {
            block_data.push_str(&nonce.to_string());

            let mut hasher = Sha3::sha3_256();
            hasher.input_str(block_data.as_str());
            let hex = hasher.result_str();

            if String::from("0000") == &hex[..4] {
                hash = hex.clone();
                break;
            } else {
                nonce += 1;
            }
        }

        info!("[CONSOLE_CALC_BLOCK] Found hash for block {}", hash);

        for (_, peer) in &config.peers {
            // TODO: Update event
            let message = MessageBuilder::new()
                .set_event_code(Events::as_val(Events::Ping))
                .set_payload(EmptyPayload::new())
                .build(&mut config.nacl, &peer.public_key);

            match socket.send_to(&message, &peer.address) {
                Ok(_)  => debug!("[CONSOLE_CALC_BLOCK] Send hash to {}", peer.address),
                Err(e) => error!("[CONSOLE_CALC_BLOCK] Error sending hash to peer: {}. Error: {}", peer.address, e),
            };
        }

        self.is_calculating = false;
        Ok(())
    }
}
use carina_core::Config;
use carina_core::Event;
use commands::console::InternalState;
use protocol_builder_parser::Parser;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub struct NewBlockContent {
    internal_state: Arc<Mutex<InternalState>>
}

impl NewBlockContent {
    pub fn new(internal_state: Arc<Mutex<InternalState>>) -> Self {
        Self {
            internal_state
        }
    }
}

impl Event for NewBlockContent {
    fn execute(&mut self, _: UdpSocket, _: String, _: &mut Config, buffer: &[u8]) {
        let parsed = Parser::parse_payload(&buffer);
        let code = Parser::to_string(&parsed[0].clone()).unwrap();
        let content = Parser::to_string(&parsed[1].clone()).unwrap();

        let mut state = self.internal_state.lock().unwrap();
        state.content.insert(code, content);

        info!("[CONSOLE_NEW_BLOCK_CONTENT] Received new content {:?}", parsed);
    }
}
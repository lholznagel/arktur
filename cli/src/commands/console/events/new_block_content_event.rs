use carina_core::Config;
use carina_core::Event;
use std::net::UdpSocket;

pub struct NewBlockContent;

impl Event for NewBlockContent {
    fn execute(&mut self, _: UdpSocket, _: String, _: &mut Config, buffer: &[u8]) {
        info!("[CONSOLE_NEW_BLOCK_CONTENT] Received new content {:?}", buffer);
    }
}
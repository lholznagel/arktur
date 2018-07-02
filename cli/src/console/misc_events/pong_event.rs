use carina_core::Event;
use carina_core::Config;
use failure::Error;
use std::net::UdpSocket;

pub struct Pong;

impl Event for Pong {
    fn execute(&mut self, _: UdpSocket, source: String, _: &mut Config, _: &[u8]) -> Result<(), Error> {
        info!("[CONSOLE_PONG] Received pong event from {:?}", source);
        Ok(())
    }
}
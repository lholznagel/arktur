use hooks::Hooks;
use std::net::UdpSocket;

pub struct Empty;

impl Hooks for Empty {
    fn on_ping(&self, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_pong(&self, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_register(&self, _: &UdpSocket, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_ack_register(&self, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_peer_registering(&self, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_new_block(&self, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_possible_block(&self, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
    fn on_found_block(&self, _: Vec<u8>, _: String) -> Vec<u8> { Vec::new() }
}
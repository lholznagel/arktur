use hooks::Hooks;
use std::net::UdpSocket;

pub struct Empty;

impl Hooks for Empty {
    fn on_ping(&self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_pong(&self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_hole_puncher(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_hole_puncher_ack(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer_ack(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_data_for_block(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_new_block(&self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_possible_block(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validate_hash(&self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validated_hash(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_found_block(&self, _: UdpSocket, _: Vec<u8>, _: String) {}
    fn on_explore_network(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {}
}
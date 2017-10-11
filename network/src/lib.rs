use std::str;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, UdpSocket, SocketAddrV6};

#[derive(Debug)]
pub struct NetworkConnect {
    ip: Ipv6Addr,
    port: u16,
}

impl NetworkConnect {
    pub fn new() -> Self {
        NetworkConnect {
            ip: Ipv4Addr::new(0, 0, 0, 0).to_ipv6_mapped(),
            port: 0
        }
    }

    pub fn start(mut self) -> UdpSocket {
        let socket = SocketAddrV6::new(self.ip, self.port, 0, 0);
        let socket = UdpSocket::bind(socket).unwrap();
        self.port = socket.local_addr().unwrap().port();
        socket
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}
use std::net::UdpSocket;

/// Stores all needed infomration about a udp client
pub struct UdpClient {
    /// open udp socket
    udp: UdpSocket
}

impl UdpClient {
    /// Creates a new UdpClient
    pub fn new(udp: UdpSocket) -> Self {
        UdpClient {
            udp: udp
        }
    }

    /// Returns the open udp connection
    pub fn connection(self) -> UdpSocket {
        self.udp
    }
}
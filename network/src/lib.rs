use std::net::{Ipv4Addr, Ipv6Addr, UdpSocket, SocketAddrV6};

#[derive(Copy, Clone, Debug)]
pub struct UdpClient {
    ip: Ipv6Addr,
    port: u16,
}

impl UdpClient {
    /// Creates a new UdpClient
    ///
    /// # Defaults
    ///
    /// Per default the port is set to 0, this forces the kernel to select a port
    /// Per default all IP-Adresses are saved as IPv6 addresses
    /// for setting a specific port call `set_port()`
    ///
    /// Per default the UDP Client listenes on 0.0.0.0
    /// for setting a specific port call `set_ip()`
    pub fn new() -> Self {
        UdpClient {
            ip: Ipv4Addr::new(0, 0, 0, 0).to_ipv6_mapped(),
            port: 0,
        }
    }

    /// Starts a UdpSocket
    ///
    /// # Return
    ///
    /// `UdpSocket` - A UdpSocket instance from `std::net::UdpSocket`
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::UdpClient;
    ///
    /// let udpClient = UdpClient::new();
    /// let udp = udpClient.start();
    ///
    /// // with calling start you get a `std::net::UdpSocket` from rust
    /// let data = [0; 10];
    /// let address = "0.0.0.0:50000";
    /// udp.send_to(&data, address).expect("couldn't send data");
    /// ```
    pub fn start(mut self) -> UdpSocket {
        let socket = SocketAddrV6::new(self.ip, self.port, 0, 0);
        let socket = UdpSocket::bind(socket).unwrap();
        self.port = socket.local_addr().unwrap().port();
        socket
    }

    /// Returns the port of the udp client
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::UdpClient;
    ///
    /// let udp_client = UdpClient::new();
    /// let udp = &udp_client.start();
    ///
    /// println!("UDP is running on port: {:?}", &udp_client.port());
    /// ```
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Sets the port for udp
    /// Must be set before calling `start()`
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::UdpClient;
    ///
    /// let udp_client = UdpClient::new();
    /// let udp_client = udp_client.set_port(50000);
    ///
    /// let udp = &udp_client.start();
    /// println!("UDP is running on port: {:?}", &udp_client.port());
    /// ```
    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Gets the ip address of the UdpClient
    /// Internaly all IP-Addresses are saved as IPv6 addresses, for IPv4 call `ip_as_ipv4()`
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::UdpClient;
    /// use std::net::Ipv4Addr;
    ///
    /// let udp_client = UdpClient::new();
    ///
    /// let udp = &udp_client.start();
    /// println!("UDP is running on ip: {:?}", &udp_client.ip());
    /// ```
    pub fn ip(&self) -> Ipv6Addr {
        self.ip
    }

    /// Wrapper for `Ipv6Addr.to_ipv4()`
    /// Please see there documentation for more information [Ipv6Addr.to_ipv4()](https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html#method.to_ipv4)
    pub fn ip_as_ipv4(&self) -> Option<Ipv4Addr> {
        self.ip.to_ipv4()
    }

    /// Sets a IPv4 address to liste on
    /// Must be called before calling `start()`
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::UdpClient;
    /// use std::net::Ipv4Addr;
    ///
    /// let udp_client = UdpClient::new();
    /// let udp_client = udp_client.set_ipv4(Ipv4Addr::new(127, 0, 0, 1));
    ///
    /// let udp = &udp_client.start();
    /// println!("UDP is running on ip: {:?}", &udp_client.ip());
    /// ```
    pub fn set_ipv4(mut self, ipv4: Ipv4Addr) -> Self {
        self.ip = ipv4.to_ipv6_mapped();
        self
    }

    /// Sets a IPv6 address to liste on
    /// Must be called before calling `start()`
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::UdpClient;
    /// use std::net::Ipv6Addr;
    ///
    /// let udp_client = UdpClient::new();
    /// let udp_client = udp_client.set_ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    ///
    /// let udp = &udp_client.start();
    /// println!("UDP is running on ip: {:?}", &udp_client.ip());
    /// ```
    pub fn set_ipv6(mut self, ipv6: Ipv6Addr) -> Self {
        self.ip = ipv6;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::UdpClient;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn start_basic_udp_client() {
        let udp_client = UdpClient::new();
        udp_client.start();
    }

    #[test]
    fn set_specific_port() {
        let udp_client = UdpClient::new().set_port(50000);
        let udp = udp_client.start();

        assert_eq!(udp_client.port, 50000);
        assert_eq!(udp_client.port, udp.local_addr().unwrap().port());
    }

    #[test]
    fn set_specific_ipv4() {
        let ip = Ipv4Addr::new(0, 0, 0, 0);
        let udp_client = UdpClient::new().set_ipv4(ip);
        let udp = udp_client.start();

        assert_eq!(ip.to_ipv6_mapped(), udp.local_addr().unwrap().ip());
    }

    #[test]
    fn set_specific_ipv6() {
        let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
        let udp_client = UdpClient::new().set_ipv6(ip);
        let udp = udp_client.start();

        assert_eq!(ip, udp.local_addr().unwrap().ip());
    }

    #[test]
    fn set_port_and_ip() {
        let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
        let udp_client = UdpClient::new().set_ipv6(ip).set_port(50000);
        let udp = udp_client.start();

        assert_eq!(udp_client.ip, udp.local_addr().unwrap().ip());
        assert_eq!(udp_client.port, udp.local_addr().unwrap().port());
    }
}
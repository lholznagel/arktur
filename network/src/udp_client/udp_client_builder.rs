use udp_client::UdpClient;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, UdpSocket, SocketAddr};

#[derive(Copy, Clone, Debug)]
/// Stores all needed information about the udp connection
pub struct UdpClientBuilder {
    /// IP-Adress, represented as a ipv6 address
    ip: IpAddr,
    /// Port the client listens to
    port: u16,
    /// handler for register requests
    register_handler: fn(String)
}

impl UdpClientBuilder {
    /// Creates a new UdpClientBuilder
    ///
    /// # Defaults
    ///
    /// Per default the port is set to 0, this forces the kernel to select a port
    /// Per default all IP-Adresses are saved as IPv6 addresses
    /// for setting a specific port call `set_port()`
    ///
    /// Per default the UDP Client listenes on 0.0.0.0
    /// for setting a specific port call `set_ip()`
    ///
    /// # Return
    ///
    /// Updated instance of the `UdpClientBuilder`
    pub fn new() -> Self {
        fn empty(value: String) {}

        UdpClientBuilder {
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 0,
            register_handler: empty
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
    /// use blockchain_network::udp_client::UdpClientBuilder;
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build();
    ///
    /// // with calling build you get a `std::net::UdpSocket` from rust
    /// let data = [0; 10];
    /// let address = "0.0.0.0:50000";
    /// udp_client.connection().send_to(&data, address);
    /// ```
    pub fn build(self) -> UdpClient {
        let socket = SocketAddr::new(self.ip, self.port);
        let socket = UdpSocket::bind(socket).unwrap();

        UdpClient::new(socket, self.register_handler)
    }

    /// Sets the port for udp
    /// Must be set before calling `start()`
    ///
    /// # Parameter
    ///
    /// `port` - New port
    ///
    /// # Return
    ///
    /// Updated instance of the `UdpClientBuilder`
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client_builder = udp_client_builder.set_port(50000);
    /// let udp_client = udp_client_builder.build();
    /// println!("UDP is running on port: {:?}", udp_client.port());
    /// ```
    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the ip address to listen on
    ///
    /// # Parameters:
    ///
    /// `ip` - IpAddr to listen on
    ///
    /// # Returns
    ///
    /// Updated version of the struct itself
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client_builder = udp_client_builder.set_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    /// let udp_client = udp_client_builder.build();
    /// println!("UDP is running on port: {:?}", udp_client.port());
    /// ```
    pub fn set_ip(mut self, ip: IpAddr) -> Self {
        self.ip = ip;
        self
    }

    /// Same as `set_ip` but takes an Ipv4Addr
    ///
    /// # Parameters:
    ///
    /// `ip` - Ipv4Addr to listen on
    ///
    /// # Returns
    ///
    /// Updated version of the struct itself
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::Ipv4Addr;
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client_builder = udp_client_builder.set_ipv4(Ipv4Addr::new(0, 0, 0, 0));
    /// let udp_client = udp_client_builder.build();
    /// println!("UDP is running on ip: {:?}", udp_client.ip());
    /// ```
    pub fn set_ipv4(self, ip: Ipv4Addr) -> Self {
        self.set_ip(IpAddr::V4(ip))
    }

    /// Same as `set_ip` but takes an Ipv6Addr
    ///
    /// # Parameters:
    ///
    /// `ip` - Ipv6Addr to listen on
    ///
    /// # Returns
    ///
    /// Updated version of the struct itself
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::Ipv6Addr;
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client_builder = udp_client_builder.set_ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    /// let udp_client = udp_client_builder.build();
    /// println!("UDP is running on ip: {:?}", udp_client.ip());
    /// ```
    pub fn set_ipv6(self, ip: Ipv6Addr) -> Self {
        self.set_ip(IpAddr::V6(ip))
    }

    /// Sets a function to call it when a register request is made
    ///
    /// # Parameters
    ///
    /// `functon` - function that should be called
    ///
    /// # Returns
    ///
    /// Updated version of the struct itself
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// 
    /// fn call_me_maybe(value: String) { }
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client_builder = udp_client_builder.set_register_handler(call_me_maybe);
    /// ```
    pub fn set_register_handler(mut self, function: fn(String)) -> Self {
        self.register_handler = function;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::UdpClientBuilder;
    use std::net::{Ipv6Addr};

    #[test]
    fn start_basic_udp_client() {
        let udp_client = UdpClientBuilder::new();
        udp_client.build();
    }

    #[test]
    fn set_specific_port() {
        let udp_client = UdpClientBuilder::new().set_port(50000);
        let udp = udp_client.build();

        assert_eq!(udp_client.port, 50000);
        assert_eq!(udp_client.port, udp.connection().local_addr().unwrap().port());
    }

    #[test]
    fn set_port_and_ip() {
        let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
        let udp_client = UdpClientBuilder::new().set_ipv6(ip).set_port(50000);
        let udp = udp_client.build();
        let udp = udp.connection();

        assert_eq!(udp_client.ip, udp.local_addr().unwrap().ip());
        assert_eq!(udp_client.port, udp.local_addr().unwrap().port());
    }
}
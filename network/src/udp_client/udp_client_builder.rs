use blockchain_hooks::HookNotification;
use udp_client::UdpClient;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, UdpSocket, SocketAddr};

#[derive(Copy, Clone, Debug)]
/// Stores all needed information about the udp connection
pub struct UdpClientBuilder {
    /// IP-Address, represented as a ipv6 address
    ip: IpAddr,
    /// Port the client listens to
    port: u16
}

impl UdpClientBuilder {
    /// Creates a new UdpClientBuilder
    ///
    /// # Defaults
    ///
    /// Per default the port is set to 0, this forces the kernel to select a port
    /// Per default all IP-Addresses are saved as IPv6 addresses
    /// for setting a specific port call `set_port()`
    ///
    /// Per default the UDP Client listenes on 0.0.0.0
    /// for setting a specific port call `set_ip()`
    ///
    /// # Return
    ///
    /// Updated instance of the `UdpClientBuilder`
    pub fn new() -> Self {
        UdpClientBuilder {
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 0
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
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    ///
    /// # fn main() {
    ///     let udp_client_builder = UdpClientBuilder::new();
    ///     let udp_client = udp_client_builder.build(HookRegister::new().get_notification());
    /// 
    ///     // with calling build you get a `std::net::UdpSocket` from rust
    ///     let data = [0; 10];
    ///     let address = "0.0.0.0:50000";
    ///     udp_client.connection().send_to(&data, address).unwrap();
    /// # }
    /// ```
    pub fn build(self, hooks: HookNotification) -> UdpClient {
        let socket = SocketAddr::new(self.ip, self.port);
        let socket = UdpSocket::bind(socket).unwrap();

        UdpClient::new(socket, hooks)
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
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    ///
    /// # fn main() {
    ///     let udp_client_builder = UdpClientBuilder::new();
    ///     let udp_client_builder = udp_client_builder.set_port(50000);
    ///     let udp_client = udp_client_builder.build(HookRegister::new().get_notification());
    ///     println!("UDP is running on port: {:?}", udp_client.port());
    /// # }
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
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// # fn main() {
    ///     let udp_client_builder = UdpClientBuilder::new();
    ///     let udp_client_builder = udp_client_builder.set_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    ///     let udp_client = udp_client_builder.build(HookRegister::new().get_notification());
    ///     println!("UDP is running on port: {:?}", udp_client.port());
    /// # }
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
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::Ipv4Addr;
    ///
    /// # fn main() {
    ///     let udp_client_builder = UdpClientBuilder::new();
    ///     let udp_client_builder = udp_client_builder.set_ipv4(Ipv4Addr::new(0, 0, 0, 0));
    ///     let udp_client = udp_client_builder.build(HookRegister::new().get_notification());
    ///     println!("UDP is running on ip: {:?}", udp_client.ip());
    /// # }
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
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::Ipv6Addr;
    ///
    /// # fn main() {
    ///     let udp_client_builder = UdpClientBuilder::new();
    ///     let udp_client_builder = udp_client_builder.set_ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    ///     let udp_client = udp_client_builder.build(HookRegister::new().get_notification());
    ///     println!("UDP is running on ip: {:?}", udp_client.ip());
    /// # }
    /// ```
    pub fn set_ipv6(self, ip: Ipv6Addr) -> Self {
        self.set_ip(IpAddr::V6(ip))
    }
}

#[cfg(test)]
mod tests {
    extern crate blockchain_hooks;

    use super::UdpClientBuilder;
    use blockchain_hooks::HookRegister;
    use std::net::Ipv6Addr;

    #[test]
    fn start_basic_udp_client() {
        let udp_client = UdpClientBuilder::new();
        udp_client.build(HookRegister::new().get_notification());
    }

    #[test]
    fn set_specific_port() {
        let udp_client = UdpClientBuilder::new().set_port(50000);
        let udp = udp_client.build(HookRegister::new().get_notification());

        assert_eq!(udp_client.port, 50000);
        assert_eq!(
            udp_client.port,
            udp.connection().local_addr().unwrap().port()
        );
    }

    #[test]
    fn set_port_and_ip() {
        let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
        let udp_client = UdpClientBuilder::new()
            .set_ipv6(ip)
            .set_port(50000);
        let udp = udp_client.build(HookRegister::new().get_notification());
        let udp = udp.connection();

        assert_eq!(udp_client.ip, udp.local_addr().unwrap().ip());
        assert_eq!(udp_client.port, udp.local_addr().unwrap().port());
    }
}
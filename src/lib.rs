use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;
#[derive(Debug)]
pub struct PortOpen {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub is_open: bool,
}
impl PortOpen {
    fn new(ip: Ipv4Addr, port: u16, is_open: bool) -> PortOpen {
        PortOpen { ip, port, is_open }
    }
}
/// Get the open status of an ipv4 port
///
/// # Examples
/// ```
/// use port_open::get_port_isopen;
/// fn main() {
///    let result = get_port_isopen(&[127, 0, 0, 1], &[80, 5037, 22]);
///    for port in result {
///        println!("{:?}", res);
///    }
/// }
/// ```
/// print-out:
/// PortOpen { ip: 127.0.0.1, port: 80, is_open: false }
/// PortOpen { ip: 127.0.0.1, port: 5037, is_open: true }
/// PortOpen { ip: 127.0.0.1, port: 22, is_open: false }
pub fn get_port_isopen(ip: &[u8; 4], ports: &[u16]) -> Vec<PortOpen> {
    let test_ip = Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]);
    let size = ports.len();
    let mut isconn: Vec<PortOpen> = Vec::with_capacity(size);
    for port in ports {
        let address = SocketAddr::new(test_ip.into(), *port);
        match TcpStream::connect_timeout(&address, Duration::from_millis(500)) {
            Ok(_) => isconn.push(PortOpen::new(test_ip, *port, true)),
            Err(_) => isconn.push(PortOpen::new(test_ip, *port, false)),
        }
    }
    isconn
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ip() {
        let res = get_port_isopen(&[127, 0, 0, 1], &[80, 5037, 22]);
        for isopen in res {
            assert_eq!(isopen.ip, Ipv4Addr::new(127, 0, 0, 1))
        }
    }
}

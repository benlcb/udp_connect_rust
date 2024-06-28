#![feature(addr_parse_ascii)]
use std::{net::{UdpSocket, Ipv4Addr, SocketAddrV4}, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let dest = Ipv4Addr::parse_ascii(args[1].as_bytes()).unwrap();
    println!("Input address: {}", dest);
    let socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).unwrap();
    socket.connect(SocketAddrV4::new(dest, 8080)).unwrap();
    let addr = socket.local_addr().unwrap();
    match addr {
        std::net::SocketAddr::V4(addr) => println!("Interface IP for route is: {}", addr.ip().to_owned()),
        _ => panic!("IPv6 adddress detected as default public address."),
    }
}

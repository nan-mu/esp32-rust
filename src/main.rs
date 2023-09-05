use std::io::{Result, Write};
use std::net::{self, Ipv4Addr, TcpStream};

fn tcp_client(&str: req, Ipv4Addr: addr) -> Result<()> {
    let client = TcpStream::connect(addr).expect("无法连接目标服务器");
    client.write_all(req);
}

fn main() {
    tcp_client(_, Ipv4Addr);
}

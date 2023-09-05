use std::io::{Result, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

fn tcp_client(req: &str, addr: &SocketAddrV4) -> Result<()> {
    let mut client = TcpStream::connect(addr).unwrap();
    client.write_all(req.as_bytes())?;
    Ok(())
}

fn main() {
    let default_ipaddr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

    tcp_client("Connect meg", &default_ipaddr).expect("连接未正常退出");
}

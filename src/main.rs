use clap::Parser;
use std::io::{Result, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

/// Simple program to send some to tcp server
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// send messege
    #[arg(short, long)]
    msg: String,

    // config server ip addr, should include port such as 127.0.0.1:8080
    #[arg(short, long)]
    ipaddr: Option<String>,
}

fn tcp_client(req: &str, addr: &SocketAddr) -> Result<()> {
    let mut client = TcpStream::connect(addr).expect("无法连接到服务器");
    client
        .write_all(req.as_bytes())
        .expect("无法向服务器发送数据");
    Ok(())
}

fn main() {
    let default_ipaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let args = Args::parse();
    let ipaddr;
    match args.ipaddr {
        // 设置自定义ip或默认ip
        None => ipaddr = default_ipaddr,
        Some(x) => ipaddr = x.parse().expect("错误的ip格式"),
    }
    tcp_client(&args.msg, &ipaddr).expect("连接未正常退出");
    println!("send {}!", args.msg);
}

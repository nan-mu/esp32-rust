use clap::Parser;
use csscolorparser;
use std::io::{Result, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

/// Simple program to send some to tcp server
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    color: Option<String>,

    /// send messege directly (for debug)
    #[arg(short, long)]
    msg: Option<String>,

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
    let args = Args::parse();
    let ipaddr;
    match args.ipaddr {
        // 设置自定义ip或默认ip
        None => ipaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        Some(x) => ipaddr = x.parse().expect("错误的ip格式"),
    };
    match args.msg {
        // 这里是为了学习命令行参数的输入和快捷调试tcp服务
        Some(msg) => {
            tcp_client(&msg, &ipaddr).expect("连接未正常退出");
            println!("send {}!", msg);
        }
        None => (),
    };
    match args.color {
        Some(def_color) => {
            let color = csscolorparser::parse(&def_color).expect("错误的颜色格式");
            let out_style_color = format!(
                "{{\"put\": RGB,\"R\":{:03},\"G\":{:03},\"B\"{:03}}}",
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8
            );
            tcp_client(&out_style_color, &ipaddr).expect("连接未正常退出");
        }
        None => (),
    }
}

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {

    let mut client = TcpStream::connect("127.0.0.1:5050").unwrap();

    client.write("client write...".as_bytes()).unwrap();

    // 定义数组并初始化数据都为 0
    let mut buffer: [u8; 1024] = [0; 1024];

    // 读取客户端流数据
    client.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));
}

use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

pub fn run(){
    // 监听 TCP 连接
    let listener = TcpListener::bind("127.0.0.1:5050").unwrap();

    // 流迭代器 - 当客户端请求时
    for stream in listener.incoming() {

        let stream = stream.unwrap();
        handle_connection(stream); // 读取请求
    }
}

// 读取请求
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
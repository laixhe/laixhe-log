use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {

    // 监听 TCP 连接
    let listener = TcpListener::bind("127.0.0.1:5050").unwrap();
    println!("Running on port 5050 ...");

    // 流迭代器 - 处理客户端请求
    for tcp_stream_result in listener.incoming() {
        match tcp_stream_result {
            Ok(tcp_stream) => {
                // 处理请求
                handle_client(tcp_stream)
            },
            Err(e) => {
                println!("客户端连接失败:{}", e);
            }
        }
    }
}

// 处理客户端请求
pub fn handle_client(mut tcp_stream: TcpStream) {
    // 定义数组并初始化数据都为 0
    let mut buffer: [u8; 1024] = [0; 1024];

    // 读取客户端流数据
    tcp_stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    // 向客户端输出
    tcp_stream.write("server write".as_bytes()).unwrap();
    // tcp_stream.flush().unwrap();
}

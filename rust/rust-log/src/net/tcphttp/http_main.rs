use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time;
use crate::net::tcphttp::thread_pool::ThreadPool;

pub fn run(){

    // 创建线程池
    let pool = ThreadPool::new(3);

    // 监听 TCP 连接
    let listener: TcpListener = TcpListener::bind("127.0.0.1:5050").unwrap();

    // 流迭代器 - 当客户端(浏览器)请求时
    for tcp_stream_result in listener.incoming() {
        match tcp_stream_result {
            Ok(tcp_stream) => {
                // 处理客户端(浏览器)请求与响应
                pool.execute(||{handle_client(tcp_stream)});
            },
            Err(e) => {
                println!("客户端连接失败:{}", e);
            }
        }
    }
}

// 处理客户端(浏览器)请求与响应
pub fn handle_client(mut tcp_stream: TcpStream) {
    // 定义数组并初始化数据都为 0
    let mut buffer: [u8; 1024] = [0; 1024];

    // 读取流数据
    tcp_stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    // 判断字符串是否以 xxx 开头
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // 一次性读取文件全部数据
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    // 向客户端(浏览器)输出
    tcp_stream.write(response.as_bytes()).unwrap();
    tcp_stream.flush().unwrap();

    // 睡眠一段时间，模拟处理时间很长
    let te = time::Duration::from_millis(10000);
    thread::sleep(te);
}
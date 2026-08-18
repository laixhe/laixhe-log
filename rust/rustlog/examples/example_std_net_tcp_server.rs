//! TCP Server 示例：监听端口并处理客户端请求。
//!
//! ## ⚠️ 运行顺序（非常重要！）
//! 1. 先在终端 1 启动 server：`cargo run --example example_std_net_tcp_server`
//! 2. 再在终端 2 启动 client：`cargo run --example example_std_net_tcp_client`
//! 3. 观察两端的输出
//!
//! 如果先启动 client，它会因为 server 没在监听而直接报错 `Connection refused`。
//!
//! ## 知识点
//! - **TcpListener**：绑定端口、等待客户端连接
//! - **incoming()**：返回一个永远迭代新连接的迭代器（阻塞模式）
//! - **TcpStream**：客户端与服务端之间的**双向字节流**（Read + Write）
//! - **read/write 返回值**：不一定读/写满整个 buffer！必须用返回的字节数判断实际处理了多少。
//!
//! 本示例是「单线程阻塞 server」——来一个处理一个，处理完才能接下一个。
//! 更高级的写法是：每个连接 `std::thread::spawn` 一个线程（多线程阻塞），或用 tokio 异步 IO。

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() {
    // 绑定 TCP 端口（0.0.0.0 表示监听本机所有网卡；仅本机测试用 127.0.0.1 更安全）
    let listener = TcpListener::bind("127.0.0.1:5050")
        // 免责：unwrap() 仅示例方便。生产中端口被占用会失败，建议打印后退出或重试
        .expect("端口 5050 绑定失败（是否已被其他程序占用？）");
    println!("✅ TCP Server 已启动，监听 127.0.0.1:5050 ...");
    println!("（另开终端运行: cargo run --example example_std_net_tcp_client）\n");

    // incoming() 产生一个阻塞迭代器：每次有新连接进来就返回 Ok(stream)
    for tcp_stream_result in listener.incoming() {
        match tcp_stream_result {
            Ok(stream) => {
                // 实际项目里这里建议：std::thread::spawn(move || handle_client(stream))
                // 来一个连接开一个线程，避免长连接阻塞后面的客户端。
                // 这里为了示例简单仍然用单线程，能看清楚整个读写流程。
                println!("==== 新客户端连接: {:?} ====", stream.peer_addr());
                handle_client(stream);
                println!("---- 连接处理完毕 ----\n");
            }
            Err(e) => {
                // 常见错误：连接建立过程中客户端 RST 等，一般打个日志继续等下一个
                eprintln!("客户端连接失败（继续监听）: {}", e);
            }
        }
    }
}

/// 处理单个客户端连接
pub fn handle_client(mut stream: TcpStream) {
    // ---- 1. 设置读写超时（生产必备！防止客户端挂死导致连接永不释放）----
    // 设置后 read/write 超过这个时间会返回 ErrorKind::TimedOut
    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(10)));

    // ---- 2. 读取客户端发送的消息 ----
    // ⚠️ 重要：TCP 是字节流，没有「消息边界」。
    // read() 返回 Ok(n)：
    //   - n > 0 : 读了 n 个字节
    //   - n = 0 : 客户端已经关闭写端（EOF）
    // 在简单 demo 里 buffer 够大能一次读完；真实协议必须循环读直到长度足够 / EOF / 超时。
    let mut buffer: [u8; 1024] = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(0) => {
            println!("客户端提前关闭了连接（0 字节，EOF）");
            return;
        }
        Ok(n) => {
            // 只展示实际读到的 n 个字节，不能打印整个 buffer（后面全是 0）
            // from_utf8_lossy：如果内容不是合法 UTF-8，用 � 替换而不是报错
            println!(
                "服务端 recv({} 字节): {}",
                n,
                String::from_utf8_lossy(&buffer[..n])
            );
        }
        Err(e) => {
            eprintln!("服务端读取失败: {}", e);
            return;
        }
    }

    // ---- 3. 给客户端返回应答 ----
    let response = b"server write: hello from tcp server!";
    match stream.write(response) {
        Ok(n) => {
            println!("服务端 send: {} 字节", n);
            // flush：确保内核发送缓冲真的发出去。
            // 大多数 TCP 实现 write 成功后会自动调度发送，这里显式 flush 更稳妥。
            if let Err(e) = stream.flush() {
                eprintln!("flush 失败: {}", e);
            }
        }
        Err(e) => eprintln!("服务端写入失败: {}", e),
    }
    // stream 离开作用域时自动 drop，TCP 连接关闭
}

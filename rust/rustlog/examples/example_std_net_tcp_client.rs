//! TCP Client 示例：连接到 server，发送数据并接收回应。
//!
//! ## ⚠️ 运行顺序（非常重要！）
//! 1. **先启动 server**：`cargo run --example example_std_net_tcp_server`
//! 2. **再启动本 client**：`cargo run --example example_std_net_tcp_client`
//!
//! 如果 server 没启动，connect 会报 `Connection refused`。
//!
//! ## 知识点
//! - `TcpStream::connect` 向服务端发起三次握手（阻塞）
//! - 同一个 TcpStream 可以先 write 再 read，因为它是全双工的
//! - 同样必须检查 write/read 的返回字节数

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() {
    println!("(请确认 server 已在运行: cargo run --example example_std_net_tcp_server)");

    // 连接 server（超时默认由 OS 控制，一般 30~120 秒）
    // 生产中可以先 connect_timeout 设置更短超时
    let mut client = TcpStream::connect("127.0.0.1:5050")
        .expect("连接 server 失败！请先运行 example_std_net_tcp_server");

    // 给流设置超时（防挂死）
    let _ = client.set_read_timeout(Some(Duration::from_secs(10)));
    let _ = client.set_write_timeout(Some(Duration::from_secs(10)));

    println!("✅ 已连接到 {:?}", client.peer_addr());

    // ---- 发送 ----
    let payload = b"client write: hello tcp server, I'm a rust client!";
    match client.write(payload) {
        Ok(n)  => println!("客户端 send: {} 字节", n),
        Err(e) => { eprintln!("写入失败: {e}"); return; }
    }
    if let Err(e) = client.flush() {
        eprintln!("flush 失败: {e}");
    }

    // ---- 接收 ----
    let mut buffer: [u8; 1024] = [0; 1024];
    match client.read(&mut buffer) {
        Ok(0) => println!("服务端未返回任何数据就关闭了连接（EOF）"),
        Ok(n) => {
            println!(
                "客户端 recv({} 字节): {}",
                n,
                String::from_utf8_lossy(&buffer[..n])
            );
        }
        Err(e) => eprintln!("读取失败: {e}"),
    }
}

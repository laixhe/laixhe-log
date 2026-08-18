#include "StdNet.h"

// Windows winsock2（必须最先包含 winsock2.h，避免与 windows.h 冲突）
#include <winsock2.h>
#include <ws2tcpip.h>

#include <chrono>     // std::chrono
#include <format>     // std::format [C++20]
#include <iostream>
#include <string>
#include <thread>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

// 本地 echo 服务端口
static constexpr int kPort = 18081;

StdNet::StdNet()
{
    // ===== 初始化 Winsock（每进程一次）=====
    WSADATA wsa;
    if (WSAStartup(MAKEWORD(2, 2), &wsa) != 0) {
        PRINT("WSAStartup 失败");
        return;
    }

    // ===== 1. 服务端：监听 + 接受连接，回显收到的数据 =====
    std::cout << "--- TCP 服务端 ---" << std::endl;

    SOCKET server = socket(AF_INET, SOCK_STREAM, 0);

    sockaddr_in addr{};
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK); // 127.0.0.1
    addr.sin_port = htons(kPort);

    // 绑定 + 监听
    if (bind(server, reinterpret_cast<sockaddr*>(&addr), sizeof(addr)) != 0) {
        PRINT("bind 失败");
        closesocket(server);
        WSACleanup();
        return;
    }
    listen(server, 1);
    PRINT("服务端已启动，监听 127.0.0.1:{}", kPort);

    // 独立线程：接受连接并回显（对应 Rust std::net::TcpListener）
    std::thread serverThread([&] {
        sockaddr_in clientAddr{};
        int len = sizeof(clientAddr);
        SOCKET client = accept(server, reinterpret_cast<sockaddr*>(&clientAddr), &len);
        if (client == INVALID_SOCKET) {
            return;
        }
        char buf[256]{};
        const int n = recv(client, buf, sizeof(buf) - 1, 0); // 接收数据
        PRINT("服务端收到: {}", std::string(buf, static_cast<std::size_t>(n)));

        send(client, buf, n, 0); // 原样回显
        closesocket(client);
    });

    // ===== 2. 客户端：连接 + 发送数据（对应 Rust TcpStream）=====
    std::cout << "--- TCP 客户端 ---" << std::endl;

    // 稍等服务端就绪
    std::this_thread::sleep_for(std::chrono::milliseconds(50));

    SOCKET client = socket(AF_INET, SOCK_STREAM, 0);
    if (connect(client, reinterpret_cast<sockaddr*>(&addr), sizeof(addr)) != 0) {
        PRINT("connect 失败");
        closesocket(client);
        closesocket(server);
        WSACleanup();
        return;
    }

    const std::string msg = "hello, tcp server!";
    send(client, msg.c_str(), static_cast<int>(msg.size()), 0);

    char echo[256]{};
    const int n = recv(client, echo, sizeof(echo) - 1, 0); // 等待回显
    PRINT("客户端收到回显: {}", std::string(echo, static_cast<std::size_t>(n)));

    closesocket(client);
    serverThread.join(); // 等待服务端线程结束
    closesocket(server);

    WSACleanup();
}

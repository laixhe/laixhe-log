#ifndef CPPAPP_STDNET_H
#define CPPAPP_STDNET_H

// 网络：winsock2 TCP 客户端 + 服务端（本地 echo）。
// 对应 Rust rustlog examples/example_std_net_tcp_server|client、Go golog http 示例。
// 注意：Windows 平台需链接 ws2_32（见 CMakeLists.txt）。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdNet
{
    public:
    StdNet();
};


#endif //CPPAPP_STDNET_H

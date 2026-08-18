#include "StdFile.h"

#include <cstdio>     // std::remove
#include <filesystem> // std::filesystem [C++17]
#include <format>     // std::format [C++20]
#include <fstream>    // ifstream / ofstream / fstream
#include <iostream>
#include <string>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdFile::StdFile()
{
    namespace fs = std::filesystem;

    // 用临时目录避免污染项目（对应 Rust std::env::temp_dir / Python tempfile）
    const std::string dir = fs::temp_directory_path().string() + "/cppapp_demo";
    fs::create_directories(dir);
    const std::string path = dir + "/data.txt";

    // ===== 1. 写入文件：ofstream（对应 Python open().write()）=====
    std::cout << "--- 写入 ---" << std::endl;

    {
        std::ofstream out(path);
        out << "hello\nworld\n";
        out.close(); // 关闭（对应 Rust RAII / Python with）
    }
    PRINT("写入成功: {}", path);

    // ===== 2. 读取文件：ifstream =====
    std::cout << "--- 读取 ---" << std::endl;

    {
        std::ifstream in(path);
        std::string line;
        std::cout << "逐行读取: ";
        while (std::getline(in, line)) {
            std::cout << line << " ";
        }
        std::cout << std::endl; // hello world
    }

    // ===== 3. 追加写入：fstream + app 模式（对应 Python open(..., "a")）=====
    std::cout << "--- 追加 ---" << std::endl;

    {
        std::ofstream out(path, std::ios::app); // 追加模式
        out << "appended\n";
        out.close();
    }
    {
        std::ifstream in(path);
        std::string content((std::istreambuf_iterator<char>(in)), std::istreambuf_iterator<char>());
        // 手动转义换行，便于观察（C++ format 无 {:?}）
        std::string escaped;
        for (char c : content) {
            escaped += (c == '\n') ? "\\n" : std::string(1, c);
        }
        PRINT("追加后内容: \"{}\"", escaped); // "hello\nworld\nappended\n"
    }

    // ===== 4. 文件存在判断与删除（对应 Python os.path.exists / os.remove）=====
    std::cout << "--- 存在与删除 ---" << std::endl;

    PRINT("文件存在: {}", fs::exists(path)); // true
    fs::remove(path);                        // 删除
    PRINT("删除后存在: {}", fs::exists(path)); // false
}

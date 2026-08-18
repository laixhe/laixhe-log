#ifndef CPPAPP_STDFILE_H
#define CPPAPP_STDFILE_H

// 文件读写：fstream 流式读写（对应 Rust rustlog file_io.rs、Go fileio_test.go）。
// 用临时文件演示，结束后自动清理。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdFile
{
    public:
    StdFile();
};


#endif //CPPAPP_STDFILE_H

#ifndef CPPAPP_STDREGEX_H
#define CPPAPP_STDREGEX_H

// 正则表达式：std::regex 匹配 / 搜索 / 替换 / 捕获分组。
// 对应 Go golog regexp_test.go、Rust rustlog examples/example_regex.rs。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdRegex
{
    public:
    StdRegex();
};


#endif //CPPAPP_STDREGEX_H

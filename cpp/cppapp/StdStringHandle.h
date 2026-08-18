#ifndef CPPAPP_STDSTRINGHANDLE_H
#define CPPAPP_STDSTRINGHANDLE_H

// 字符串进阶：查找 / 替换 / 子串 / 分割 / 大小写 / 词频统计。
// 对应 Go golog string_test.go（strings 包）、TS tslog strings.test.ts。
// 基础字符串（原始字面量、format）见 StdString。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdStringHandle
{
    public:
    StdStringHandle();
};


#endif //CPPAPP_STDSTRINGHANDLE_H

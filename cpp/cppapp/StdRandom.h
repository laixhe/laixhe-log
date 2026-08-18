#ifndef CPPAPP_STDRANDOM_H
#define CPPAPP_STDRANDOM_H

// 随机数：std::random_device / 生成器 / 分布 / 打乱。
// 对应 Rust rustlog examples/example_rand.rs、Python random 模块。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdRandom
{
    public:
    StdRandom();
};


#endif //CPPAPP_STDRANDOM_H

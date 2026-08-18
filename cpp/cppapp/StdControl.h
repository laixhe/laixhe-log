#ifndef CPPAPP_STDCONTROL_H
#define CPPAPP_STDCONTROL_H

// 控制流：if / for / while / do-while / switch。
// 对应 Go golog control_flow_test.go、Rust rustlog control_flow.rs。

// 统一设计：构造函数内依次运行该主题全部演示，main.cpp 中实例化即输出；
// 用类而非函数，便于在 main() 中按行开关单个模块（注释掉实例化行即可跳过）。
class StdControl
{
    public:
    StdControl();
};


#endif //CPPAPP_STDCONTROL_H

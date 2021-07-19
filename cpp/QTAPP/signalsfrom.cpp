#include "signalsfrom.h"

// 自定义信号 signals
//
// 信号是类的成员函数
// 要使用 signals 关键字进行声明，使用方法与 public 类似
// 返回值是 void 类型
// 信号函数只需要声明，不需要定义(没有函数体实现)
// 发送信号函数前加关键字： emit

SignalsFrom::SignalsFrom(QObject *parent) : QObject(parent) {
}

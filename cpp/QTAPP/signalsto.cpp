#include <iostream>

#include "signalsto.h"

// 自定义槽 slots
//
// 槽函数就是信号的处理动作，自定义槽函数和自定义的普通函数一样
// 返回值是 void 类型

SignalsTo::SignalsTo(QObject *parent) : QObject(parent) {
}

void SignalsTo::testTo(){
    std::cout << "SignalsFrom::testFrom -> SignalsTo::testTo..." << std::endl;
}

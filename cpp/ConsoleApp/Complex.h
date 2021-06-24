#ifndef __COMPLEX_H
#define __COMPLEX_H

#include <iostream>

class Complex {
public:
    Complex();
    Complex(double r, double i);
    virtual ~Complex();
    // 拷贝构造函数
    Complex(const Complex& x);

    double GetReal() const;
    void SetReal(double r);
    double GetImage() const;
    void GetImage(double i);

    // 运算符重载 +
    Complex operator+(const Complex& x);
    // 运算符重载 =
    Complex& operator=(const Complex& x);
    // 运算符重载 ++
    Complex& operator++();   // 前置++
    Complex operator++(int); // 后置++

    // 友元机制允许类的非公有成员被一个类或者函数访问
    // 友元可以是一个函数，该函数被称为友元函数；友元也可以是一个类，该类被称为友元类
    // 标准输入输出IO重载
    friend std::ostream& operator<<(std::ostream& out, const Complex& x); // 运算符重载 <<
    friend std::istream& operator>>(std::istream& put, Complex& x);       // 运算符重载 >>

private:
    double m_real; // 复数的实部
    double m_image; // 复数的虚部
};


#endif // __COMPLEX_H

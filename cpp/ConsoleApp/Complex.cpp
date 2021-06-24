#include "Complex.h"

Complex::Complex(){
    m_real = 0;
    m_image = 0;
}
Complex::Complex(double r, double i) {
    m_real = r;
    m_image = i;
}

Complex::~Complex() {

}

// 拷贝构造函数
Complex::Complex(const Complex &x) {
    m_real = x.m_real;
    m_image = x.m_image;
}

double Complex::GetReal() const {
    return m_real;
}

void Complex::SetReal(double r) {
    m_real = r;
}

double Complex::GetImage() const {
    return m_image;
}
void Complex::GetImage(double i) {
    m_image = i;
}

// 运算符重载 +
Complex Complex::operator+(const Complex &x) {
    return Complex(m_real+x.m_real, m_image+x.m_image);
}

// 运算符重载 =
Complex& Complex::operator=(const Complex& x) {
   if (this != &x){
       m_real = x.m_real;
       m_image = x.m_image;
   }
    return *this;
}

// 运算符重载 前置++
Complex& Complex::operator++() {
    m_real++;
    m_image++;
    return *this;
}

// 运算符重载 后置++
Complex Complex::operator++(int) {
    return Complex(m_real++, m_image++);
}

// 运算符重载 <<
std::ostream& operator<<(std::ostream& out, const Complex& x){
    out << "Complex(" << x.m_real << "," << x.m_image << ")";
    return out;
}

// 运算符重载 >>
std::istream& operator>>(std::istream& put, Complex& x) {
    put >> x.m_real >> x.m_image;
    return put;
}

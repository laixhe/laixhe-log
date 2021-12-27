#ifndef __MYSQLX_H
#define __MYSQLX_H

#include <iostream>
#include "mysql.h"

class Mysqlx {
public:
    // 禁用 拷贝构造函数
    Mysqlx(const Mysqlx&)=delete;
    // 禁用 = 运算符重载
    Mysqlx& operator=(const Mysqlx&)=delete;

    // 注意返回的是引用
    static Mysqlx& getInstance();

    bool initialize(std::string host,  std::string dbname, unsigned int port, std::string user, std::string pwd);

private:
    MYSQL* m_mysql;

    Mysqlx();
    ~Mysqlx();
};

#endif // __MYSQLX_H
#include "Mysqlx.h"

Mysqlx::Mysqlx(){
    m_mysql = nullptr;
    std::cout << "Mysqlx::Mysqlx()" << std::endl;
}

Mysqlx::~Mysqlx(){
    std::cout << "Mysqlx::~Mysqlx()" << std::endl;
}

// 利用局部静态变量只会初始化一次，而且还是线程安全的
// 注意返回的是引用
// C++11 之前不能这么写
Mysqlx& Mysqlx::getInstance() {
    // 局部静态变量
    static Mysqlx instance;
    return instance;
}

bool Mysqlx::initialize(std::string host, std::string dbname, unsigned int port, std::string user, std::string passwd){
    // 初始化上下文
    m_mysql = mysql_init(m_mysql);
    // 连接登录数据库
    m_mysql = mysql_real_connect(m_mysql, host.c_str(), user.c_str(), passwd.c_str(), dbname.c_str(), port, nullptr, 0);

    if (m_mysql == nullptr) {
        return false;
    }

    mysql_query(m_mysql, "set names utf8mb4");
    
    return true;
}


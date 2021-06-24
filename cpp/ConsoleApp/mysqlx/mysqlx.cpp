#include <iostream>
#include "mysql.h"

void initDB(){

    // 初始化上下文
    MYSQL mysql;
    mysql_init(&mysql);

    const char *host = "192.168.10.240";
    const char *user = "root";
    const char *pass = "123456";
    const char *db = "test";

    // g++ mysql.cpp -l mariadbclient
    // 连接登录数据库
    if(!mysql_real_connect(&mysql, host, user, pass, db, 3306, 0, 0)){
        std::cout << "mysql connect failed!" << mysql_error(&mysql) << std::endl;
    } else {
        std::cout << "mysql connect success!" << std::endl;
    }

}
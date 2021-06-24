#include <iostream>
#include <array>
#include <vector>
#include <map>
#include <set>
#include <tuple>

#include "array_list_map.h"

// 数组
void _array() {

    // 声明并初始化数组
    int arrInt[3] = {}; // 全部赋值为 0
    for(int i=0; i<3; i++) {
        std::cout << "arrInt=" << arrInt[i] << std::endl;
    }

    std::cout << "------------------" << std::endl;

    // 声明并初始化数组
    int arrayInt[] = {1, 2, 3}; // 自动识别长度
    for(int i=0; i<3; ++i) {
        std::cout << "arrayInt=" << arrayInt[i] << std::endl;
    }

    // 数组引用
    int (&ref)[3] = arrInt;
}

// array 与数组一样，array 对象的长度也是固定的，
// 也使用栈(静态内存分配)，而不是自由存储区，因此其效率与数组相同，但更方便，更安全
void std_array() {
    auto arrInt = std::array<int, 3>();

    arrInt[0] = 1;
    arrInt[1] = 2;
    arrInt[2] = 3;

    std::cout << "------------------" << std::endl;

    // foreach 写法
    for(auto i : arrInt) {
        std::cout << "std_array - arrInt=" << i << std::endl;
    }

    std::cout << "------------------" << std::endl;

    // 迭代
    for (auto i = arrInt.begin(); i != arrInt.end(); ++i) {
        std::cout << "std_array - arrInt=" << *i << std::endl;
    }

}

// vector 向量类型(动态数组)
// 大小变化:1024  2048  4096（开始会以2倍数增加，后面慢慢以1/3、1/5等的形式增加）
//
// push_back()     追加元素
// emplace_back()  追加元素 (比 push_back 更有效率)(C++11)
// back()       返回最后一个元素
// front()      返回第一个元素
// pop_back()   删除最后一个元素
// clear()      清除所有元素
// empty()      判断是否为空
// size()       返回元素个数(长度)
// capacity()   返回容纳的元素个数(容量)
// begin()      返回指向容器第一个元素的迭代器
// end()        返回指向容器最后一个元素的迭代器
// insert()     插入元素
// emplace()    插入元素(比 insert 更有效率)(C++11)
void std_vector() {

    auto arrInt = std::vector<int>();
    arrInt.push_back(1);
    arrInt.push_back(2);
    arrInt.emplace_back(3);

    std::cout << "------------------" << std::endl;

    std::cout << "std_vector - arrInt.size = " << arrInt.size() << std::endl;
    std::cout << "std_vector - arrInt.capacity = " << arrInt.capacity() << std::endl;

    std::cout << "------------------" << std::endl;

    // foreach 写法
    for (auto i : arrInt) {
        std::cout << "std_vector - arrInt=" << i << std::endl;
    }

    std::cout << "------------------" << std::endl;

    // 删除
    arrInt.pop_back(); // 删除末尾

    // 迭代
    for (auto i = arrInt.begin(); i != arrInt.end(); ++i) {
        std::cout << "std_vector - arrInt=" << *i << std::endl;
    }
    
}

// map 关联容器
// insert()   插入元素
// erase()    删除元素
// find()     查找元素
// clear()    清除所有元素
// size()     返回元素个数
// empty()    判断是否为空
// begin()    返回指向容器第一个元素的迭代器
// end()      返回指向容器最后一个元素的迭代器
void std_map(){
    std::map<std::string, std::string> mapData;

    mapData["name"] = "laixhe";
    mapData["age"] = "18";

    std::cout << "------------------" << std::endl;

    std::cout << "std_map - name=" << mapData["name"] << " age=" << mapData["age"]<< std::endl;

    std::cout << "------------------" << std::endl;

    // foreach 写法
    for (auto item : mapData) {
        std::cout << "std_map - k=" << item.first << " v=" << item.second << std::endl;
    }

    std::cout << "------------------" << std::endl;

    // 删除
    mapData.erase("name");

    // 迭代
    for (auto item = mapData.begin(); item != mapData.end(); ++item) {
        std::cout << "std_map - k=" << item->first << " v=" << item->second << std::endl;
    }


    std::cout << "------------------" << std::endl;

    // 查找元素
    auto age = mapData.find("age");
    if(age != mapData.end()){
        std::cout << "std_map - age-k=" << age->first << " age-v=" << age->second << std::endl;
    }
}

// set 顺序容器
// 值是唯一的
// insert()   插入元素
// erase()    删除元素
// clear()    清除所有元素
// count()    返回元素个数
// empty()    判断是否为空
// begin()    返回指向容器第一个元素的迭代器
// end()      返回指向容器最后一个元素的迭代器
void std_set() {
    auto setData = std::set<int>();
    setData.insert(1);
    setData.insert(11);
    setData.insert(111);
    setData.insert(1);

    std::cout << "------------------" << std::endl;

    std::cout << "std_set - setData.size = " << setData.size() << std::endl;

    std::cout << "------------------" << std::endl;

    // foreach 写法
    for (auto i : setData) {
        std::cout << "std_set - setData=" << i << std::endl;
    }

    std::cout << "------------------" << std::endl;

    // 删除
    setData.erase(111);

    // 迭代
    for (auto i = setData.begin(); i != setData.end(); ++i) {
        std::cout << "std_set - setData=" << *i << std::endl;
    }
}

// tuple 元组
// C++ 11 引入
void std_tuple() {
    std::tuple<int, int, std::string> tupleData = std::make_tuple(18, 19, "laixhe");

    std::cout << "------------------" << std::endl;

    std::cout << "std_tuple - tupleData.0=" << std::get<0>(tupleData) << std::endl;
    std::cout << "std_tuple - tupleData.1=" << std::get<1>(tupleData) << std::endl;
    std::cout << "std_tuple - tupleData.2=" << std::get<2>(tupleData) << std::endl;


    std::cout << "------------------" << std::endl;

    // 修改
    std::get<2>(tupleData) = "lai";
    std::cout << "std_tuple - tupleData.2=" << std::get<2>(tupleData) << std::endl;

}

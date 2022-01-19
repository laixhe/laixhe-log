#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <list>
#include <thread>
#include <memory>

// [C++11]
// 头文件 <thread>
// 程序运行起来，生成一个进程和所属的主线程，主线程从 main() 开始执行，
// 那么我们自已创建的线程，也需要从一个函数开始运行(初始函数)，
// 一旦这个函数运行完毕，就代表这个线程运行结束。
//
// 创建一个线程一般会对参数进行这个几个步骤：[主函数 拷贝到 线程临时 拷贝到 参数中]
// 
// 线程的方法和属性
/*
1. joinable() 判断线程是否可连接(可执行线程)的，有以下情况的，为不可连接的：
    1. 构造时 thread() 没有参数
    2. 该对象的线程已经被移动了
    3. 该线程已经被 join() 或 detach()
    4. get_id() 返回线程的ID ( std::this_thread::get_id() )

2. join()   等待线程执行完成，使用 join 就不能用 detach 了

3. detach() 分离线程，分离后对象不再拥有线程，
            该线程结束后，会自动回收内存(并不会开启另一个进程)，
            使用 detach 就不能用 join 了

4. native_handle() 返回 POSIX 标准的线程对象

5. swap() 交换对象的线程
*/

// std::this_thread::get_id()    获取当前线程ID
// std::this_thread::sleep_for() 将某个线程阻塞(休眠)

// 仿函数(可调用对象)
class std_thread_main_class{
public:
    // 创建线程函数(初始函数)，不能带参数
    void operator()(){
        std::cout << "线程:std_thread_main_class::operator()..." << std::endl;
        std::cout << "线程:std_thread_main_class end --------------" << std::endl;
    }
};

// 创建线程函数(初始函数)
void std_thread_main_print(){
    std::cout << "线程:std_thread_main_print..." << std::endl;
    std::cout << "线程:std_thread_main_print end --------------" << std::endl;
}

// 创建线程
void std_thread_main(){

    // 创建一个线程
    std::thread obj(std_thread_main_print);
    // 判断是否没用过 join()
    if (obj.joinable()) {
        // 等待线程执行完成
        obj.join();
    }

    std::cout << "------------------" << std::endl;

    // 用类对象(可调用对象)，创建一个线程
    std_thread_main_class stp;
    std::thread obj1(stp); // 在创建线程时，会复制 stp 对象到线程中(调用了拷贝构造函数)
    if (obj1.joinable()) {
        obj1.join();
    }

    std::cout << "------------------" << std::endl;

    // 用 lambda 表达式，创建一个线程
    auto lambda_thread = []{
        std::cout << "线程:lambda_thread..." << std::endl;
        std::cout << "线程:lambda_thread end --------------" << std::endl;
    };
    std::thread obj2(lambda_thread);
    if (obj2.joinable()) {
        obj2.join();
    }

    std::cout << "------------------" << std::endl;
    std::cout << "std_thread_main end --------------" << std::endl;
}

// -----------------------------------------

// 创建线程函数(初始函数)
// 参数不要用 指针
// 参数要辟免隐式转换
// 参数是类对象建议用 引用 (否则会多调用一次拷贝构造函数 [主函数 拷贝到 线程临时 拷贝到 参数中])
void std_thread_print_var(const int i, const std::string &buf){
    std::cout << "线程:std_thread_print_var i=" << i << std::endl;
    std::cout << "线程:std_thread_print_var buf=" << buf << std::endl;

    std::cout << "线程:std_thread_print_var end --------------" << std::endl;
}

void std_thread_print_var_main(){

    // 传递临时对象作为线程参数
    int mvar = 1;
    char buf[] = "this is a test!";
    // 会按照顺序参数传到 my_print函数 里去
    // 辟免隐式转换(不要用 char* 去隐式转换为 string，应要显示转换为 string)
    std::thread obj(std_thread_print_var, mvar, std::string(buf));
    obj.join();

    std::cout << "std_thread_print_var_main end --------------" << std::endl;
}

// -----------------------------------------

class std_thread_class{
public:
    int m_i = 10;

    std_thread_class(){
        std::cout << "std_thread_class 构造函数 m_i=" << m_i << " id=" << std::this_thread::get_id() <<  std::endl;
    }

    // 拷贝构造函数
    std_thread_class(const std_thread_class& x){
        std::cout << "std_thread_class 拷贝构造函数 m_i=" << m_i << " id=" << std::this_thread::get_id() << std::endl;
    }

    ~std_thread_class(){
        std::cout << "std_thread_class 析构函数 m_i=" << m_i << " id=" << std::this_thread::get_id() << std::endl;
    }

};

// 创建线程函数(初始函数)
void std_thread_class_print(std_thread_class &c){

    c.m_i = 199;

    std::cout << "线程:std_thread_class_print end thread_id=" << std::this_thread::get_id() << std::endl;
}

void std_thread_class_main(){
    std::cout << "std_thread_class_main thread_id=" << std::this_thread::get_id() << std::endl;
    std::cout << "------------------" << std::endl;

    std_thread_class c;

    // std::ref  用于包装按引用传递的值 (不需要调用 拷贝构造函数 了)
    std::thread obj(std_thread_class_print, std::ref(c));
    obj.join();

    std::cout << "------------------" << std::endl;
    std::cout << "std_thread_class m_i=" << c.m_i << std::endl;

    std::cout << "std_thread_class_main end --------------" << std::endl;
}


// -----------------------------------------

// 创建线程函数(初始函数)
void std_thread_uniquer(std::unique_ptr<int> p){
    std::cout << "线程:std_thread_uniquer p=" << *p << std::endl;
    std::cout << "线程:std_thread_uniquer end thread_id=" << std::this_thread::get_id() << std::endl;
}

void std_thread_uniquer_main(){

    std::unique_ptr<int> p(new int(100));

    std::thread obj(std_thread_uniquer, std::move(p));
    obj.join(); // 不能使用 detach()

    std::cout << "------------------" << std::endl;
    std::cout << "std_thread_uniquer_main end --------------" << std::endl;
}

// -----------------------------------------

class std_thread_class_func{
public:
    int m_i = 10;

    std_thread_class_func(){
        std::cout << "std_thread_class_func 构造函数" << this << " m_i=" << m_i << " id=" << std::this_thread::get_id() <<  std::endl;
    }

    // 拷贝构造函数
    std_thread_class_func(const std_thread_class_func& x){
        std::cout << "std_thread_class_func 拷贝构造函数" << this << " m_i=" << m_i << " id=" << std::this_thread::get_id() << std::endl;
    }

    ~std_thread_class_func(){
        std::cout << "std_thread_class_func 析构函数" << this << " m_i=" << m_i << " id=" << std::this_thread::get_id() << std::endl;
    }

    // 创建线程函数(初始函数)
    void thread_work(){
      std::cout << "线程:std_thread_class_func::thread_work()" << this << " m_i=" << m_i << " id=" << std::this_thread::get_id() << std::endl;
    }

};

void std_thread_class_func_main(){

    std_thread_class_func f;

    std::thread obj(&std_thread_class_func::thread_work, std::ref(f));
    obj.join(); // 不能使用 detach()

    std::cout << "------------------" << std::endl;
    std::cout << "std_thread_class_func_main end --------------" << std::endl;
}

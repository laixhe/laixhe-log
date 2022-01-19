#include <iostream>
#include <string>
#include <list>
#include <chrono>
#include <thread>
#include <future>

// [C++11]
// std::async 异步运行任务（有可能在新线程中执行），并返回保有其结果的 std::future
// 头文件 <future>

// 创建线程函数(初始函数) 
int std_thread_async_future_thread() {
    std::cout << "std_thread_async_future_thread start" << " thread_id=" << std::this_thread::get_id() << std::endl;

    // 休眠 2 秒
    std::chrono::seconds dura(2);
    std::this_thread::sleep_for(dura);

    std::cout << "std_thread_async_future_thread end" << " thread_id=" << std::this_thread::get_id() << std::endl;
    return 20;
}

void std_thread_async_future_main() {

    // 创建异步运行任务(线程)
    std::future<int> result = std::async(std_thread_async_future_thread);

    // 程序会阻塞 直到 任务处理完成
    int result_i = result.get(); // 只能调用一次，不能调用多次

    std::cout << "std_thread_async_future_main result_i=" << result_i << " thread_id=" << std::this_thread::get_id() << std::endl;
    
    std::cout << "std_thread_async_future_main end" << " thread_id=" << std::this_thread::get_id() << std::endl;
}

// -----------------------------------------

class std_thread_async_future_class{
public:
    std_thread_async_future_class(){
        std::cout << "std_thread_async_future_class 构造函数" << this << " thread_id=" << std::this_thread::get_id() << std::endl;
    }

    ~std_thread_async_future_class(){
        std::cout << "~std_thread_async_future_class 析构函数" << this << " thread_id=" << std::this_thread::get_id() << std::endl;
    }

    std_thread_async_future_class(const std_thread_async_future_class& x){
        std::cout << "std_thread_async_future_class 拷贝构造函数" << this << " thread_id=" << std::this_thread::get_id() << std::endl;
    }

    // 创建线程函数(初始函数)
    int thread_work(int i) {
        std::cout << "thread_work_set start " << this << " i=" << i << " thread_id=" << std::this_thread::get_id() << std::endl;
        std::cout << "thread_work_set end " << this << " i=" << i << " thread_id=" << std::this_thread::get_id() << std::endl;
        return 100;
    }

};

void std_thread_async_future_class_main(){

    std_thread_async_future_class obj;
    // 创建异步运行任务(线程)
    std::future<int> result = std::async(&std_thread_async_future_class::thread_work, std::ref(obj), 111);

    std::future_status status = result.wait_for(std::chrono::seconds(1)); // 等待 1 秒
    if (status == std::future_status::timeout) {
        // 超时：表示线程还没执行完
    }

    // 程序会阻塞 直到 任务处理完成
    int result_i = result.get();

    std::cout << "std_thread_async_future_class_main result_i=" << result_i << " thread_id=" << std::this_thread::get_id() << std::endl;
    
    std::cout << "std_thread_async_future_class_main end" << " thread_id=" << std::this_thread::get_id() << std::endl;
}

// -----------------------------------------

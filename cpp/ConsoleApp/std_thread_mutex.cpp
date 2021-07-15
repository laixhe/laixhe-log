#include <iostream>
#include <string>
#include <list>
#include <thread>
#include <mutex>
#include <condition_variable>

// [C++11]
// 多线程与互斥量(锁)的运用
// std::lock_guard 类模板，用于替代 std::mutex 的 lock() 和 unlock()
// std::unique_lock 为锁管理模板类
// 头文件：<mutex>
//
// std::condition_variable 条件变量
// 条件变量允许我们通过通知进而实现线程同步(理解为：发送方/接收方或生产者/消费者之类的工作流)
// 头文件： <condition_variable>

class std_thread_many_print{
public:

    std_thread_many_print(){
        std::cout << "std_thread_many_print构造函数" << this << " thread_id=" << std::this_thread::get_id() << std::endl;
    }

    ~std_thread_many_print(){
        std::cout << "~std_thread_many_print析构函数" << this << " thread_id=" << std::this_thread::get_id() << std::endl;
    }

    std_thread_many_print(const std_thread_many_print& x){
        std::cout << "std_thread_many_print 拷贝构造函数" << this << " thread_id=" << std::this_thread::get_id() << std::endl;
    }

    // 创建线程函数(初始函数) - 存数据
    void thread_work_set(){
        std::cout << "thread_work_set start" << this << " thread_id=" << std::this_thread::get_id() << std::endl;

        // for(int i=0; i < 100000; ++i){
        //     data_mutex.lock(); // 加锁
        //     data.push_back(i);
        //     std::cout << "thread_work_set" << this << " i=" << i << " thread_id=" << std::this_thread::get_id() << std::endl;
        //     data_mutex.unlock(); // 解锁
        // }

        // for(int i=0; i < 100000; ++i){
        //     std::lock_guard<std::mutex> data_lock(data_mutex); // 自动加锁和解锁
        //     data.push_back(i);
        //     std::cout << "thread_work_set" << this << " i=" << i << " thread_id=" << std::this_thread::get_id() << std::endl;
        // }

        for(int i=0; i < 100000; ++i){
            std::cout << "thread_work_set" << this << " i=" << i << " thread_id=" << std::this_thread::get_id() << std::endl;

            std::unique_lock<std::mutex> data_lock(data_mutex); // 自动加锁和解锁
            data.push_back(i);
            data_cond.notify_one(); // 条件通知( 唤醒 wait() )
        }

        std::cout << "thread_work_set end" << this << " thread_id=" << std::this_thread::get_id() << std::endl;
    }

    // 创建线程函数(初始函数) - 取数据
    void thread_work_get(){
        std::cout << "thread_work_get start" << this << " thread_id=" << std::this_thread::get_id() << std::endl;

        // for(int i=0; i < 100000; ++i){
        //     data_mutex.lock(); // 加锁
        //     if(!data.empty()){
        //
        //         int cmd = data.front();
        //         data.pop_front();
        //         std::cout << "thread_work_get" << this << " cmd=" << cmd << " i="<< i << " thread_id=" << std::this_thread::get_id() << std::endl;
        //
        //     } else {
        //         std::cout << "thread_work_get" << this << " data 数据为空... i=" << i << " thread_id=" << std::this_thread::get_id() << std::endl;
        //     }
        //     data_mutex.unlock(); // 解锁
        // }

        // for(int i=0; i < 100000; ++i){
        //     std::lock_guard<std::mutex> data_lock(data_mutex); // 自动加锁和解锁
        //     if(!data.empty()){
        //
        //         int cmd = data.front();
        //         data.pop_front();
        //         std::cout << "thread_work_get" << this << " cmd=" << cmd << " i="<< i << " thread_id=" << std::this_thread::get_id() << std::endl;
        //
        //     } else {
        //         std::cout << "thread_work_get" << this << " data 数据为空... i=" << i << " thread_id=" << std::this_thread::get_id() << std::endl;
        //     }
        // }
        

        while (true){
            std::unique_lock<std::mutex> data_lock(data_mutex); // 自动加锁和解锁
            // wait 等待通知
            // 如果第二个参数 lambda 表达式返回值是 false，那么 wait() 将解锁互斥量，
            // 并堵塞到本行，堵塞到其它某个线程调用 notify_one() 成员函数为止，
            // 如果返回 true，则将锁互斥量与不堵塞
            // 如果没有第二个参数默认为 false (跟 lambda 表达式返回值是 false 一样)
            data_cond.wait(data_lock, [this]{
                if(!data.empty()){
                    return true;
                }
                return false;
            });

            // 流程只要能走到这里来，互斥锁一定是锁的，且有数据的

            // if(!data.empty()){
            //
            //     int cmd = data.front();
            //     data.pop_front();
            //     std::cout << "thread_work_get" << this << " cmd=" << cmd << " thread_id=" << std::this_thread::get_id() << std::endl;
            //
            // } else {
            //     std::cout << "thread_work_get" << this << " data 数据为空..." << " thread_id=" << std::this_thread::get_id() << std::endl;
            // }

            // 可能存在多条数据
            while (!data.empty()){
                int cmd = data.front();
                data.pop_front();
                std::cout << "thread_work_get" << this << " cmd=" << cmd << " thread_id=" << std::this_thread::get_id() << std::endl;
            }
            data_lock.unlock(); // 因为 unique_lock 的灵活性，所以可以随时 unlock 解锁，以免锁住太长时间 (可不用主动解锁的，会自动解锁)
            
        }

        std::cout << "thread_work_get end" << this << " thread_id=" << std::this_thread::get_id() << std::endl;
    }

private:
    std::list<int> data;   // 数据
    std::mutex data_mutex; // 锁
    std::condition_variable data_cond; // 条件(通知)
};

void std_thread_many_main(){

    std_thread_many_print obj;

    std::thread getObj(&std_thread_many_print::thread_work_get, std::ref(obj));
    std::thread setObj(&std_thread_many_print::thread_work_set, std::ref(obj));

    getObj.join();
    setObj.join();

    std::cout << "------------------" << std::endl;
    std::cout << "std_thread_many_main end --------------" << std::endl;
}

// -----------------------------------------

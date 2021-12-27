using System;
using System.Threading;

// .net3.5 的
// 多线程

namespace ConsoleApp {
    class Thread_ {
        public static void run() {

            // 创建线程
            Thread t1 = new Thread(ThreadStart);
            // 执行线程
            t1.Start();

            // 创建线程 - 匿名函数
            Thread t2 = new Thread(()=>{
                Console.WriteLine("()=>{}");
            });
            // 执行线程
            t2.Start();

            // 等待线程执行完毕
            t2.Join();

        }

        public static void ThreadStart(){
            Console.WriteLine("ThreadStart...");
        }
    }
}
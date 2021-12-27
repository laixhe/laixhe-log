using System;
using System.Threading;
using System.Threading.Tasks;

// .net4.0 的
// 多线程任务

namespace ConsoleApp {
    class ThreadTask {

        public static CancellationTokenSource ct = new CancellationTokenSource();

        public static void run() {

            // 执行多线程任务
            Task t1 = Task.Run(TaskStart);

            // 计时等待
            var t2 = Task.Run(TaskDelay);

            // 等待线程的执行完毕
            //t1.Wait();
            //t2.Wait();

            // Task 任务的取消
            Task.Run(TaskSource, ct.Token);

            Console.WriteLine("ThreadTask.run...");
        }

        public static void TaskStart(){
            for (int i = 0; i < 10; i++){
                Console.WriteLine("TaskStart... {0}", i);
            }
        }

        // 异步操作
        public static async void TaskDelay(){
            int i = 0;
            while (true) {
                Console.WriteLine("TaskDelay... {0}", i);
                // Task.Delay 可以执行一个延时的操作
                await Task.Delay(1000); // 单位毫秒
                if (i >= 3) {
                    // 通知 TaskSource 取消任务的信号
                    ct.Cancel();
                }
                ++i;
            }
        }

        public static async void TaskSource(){
            // ct.IsCancellationRequested
            // true 已经接收到取消的信号
            // false 表示没有收到取消任务信号
            while (ct.IsCancellationRequested == false){
                Console.WriteLine("TaskSource...");
                await Task.Delay(1000); // 延时单位毫秒
            }
        }
        
    }
}
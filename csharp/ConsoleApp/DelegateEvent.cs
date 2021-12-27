using System;

// 事件 配合 委托
// 而事件又是回调机制的一种应用
// 可看作函数数组

namespace ConsoleApp {

    // 1. 定义委托
    public delegate void AddHandler();

    class DelegateEvent {

        // 2. 定义事件
        public static event AddHandler AddEvent;

        // 3. 处理事件
        public static void start(){
            if (AddEvent != null){
                // 执行
                AddEvent();
            }
        }

        public static void run() {
            var a = new AddDouble{
                x=1,
                y=2
            };
            var b = new AddDouble{
                x=3,
                y=4
            };
            
            // 4.添加委托
            AddEvent += new AddHandler(a.show);
            AddEvent += new AddHandler(b.show);

            // 3. 处理事件
            start();
        }
    }

    class AddDouble{
        public double x;
        public double y;

        public void show(){
            Console.WriteLine("x={0}, y={1}", this.x, this.y);
        }
    }
}
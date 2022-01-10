using System;

// 事件 配合 委托
// 而事件又是回调机制的一种应用
// 可看作函数数组

namespace CsharpLog
{

    // 1. 定义委托
    public delegate void AddHandler();

    class DelegateEvent {

        // 2. 定义事件
        public static event AddHandler AddEvent;

        // 3. 处理事件
        public static void Start(){
            if (AddEvent != null){
                // 执行
                AddEvent();
            }
        }

        public static void Run() {
            var a = new AddDouble{
                x=1,
                y=2
            };
            var b = new AddDouble{
                x=3,
                y=4
            };
            
            // 4.添加委托
            AddEvent += new AddHandler(a.Show);
            AddEvent += new AddHandler(b.Show);

            // 3. 处理事件
            Start();
        }
    }

    class AddDouble{
        public double x;
        public double y;

        public void Show(){
            Console.WriteLine("x={0}, y={1}", this.x, this.y);
        }
    }
}
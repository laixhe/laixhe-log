using System;

// 委托是一个类
// 将方法作为 参数
// 是实现回调函数的一种机制

namespace CsharpLog
{

    // 1. 定义委托
    public delegate double Add(double x, double y);

    class Delegate {
        public static void Run() {

            // 3. 实例委托
            Add addD = new Add(addition);

            // 4. 使用委托
            double d = addD(3, 5);

            Console.WriteLine("d={0}", d); // 结果：8

            // 匿名方法 配合 委托
            Add addE = delegate(double x, double y){
                return x + y;
            };
            double e = addE(30, 5);
            Console.WriteLine("e={0}", e); // 结果：35

        }

        // 2. 方法
        public static double addition(double x, double y){
            return x + y;
        }
    }

}
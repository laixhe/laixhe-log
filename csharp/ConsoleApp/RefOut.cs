using System;

// ref
// 1. 作用
// 将一个变量传入一个函数中进行*处理*，*处理*完成后，再将*处理*后的值带出函数
//
// 2. 要求
// 函数外必须为变量赋值，而函数内可以不赋值
//
// 3. 语法
// 形参和实参前面都要加 ref 关键字
//
// out
// 1. 作用
// 一个函数中如果返回多个不同类型的值，就需要用到 out 参数
//
// 2. 语法
// 形参和实参前面都要加 out 关键字
//

namespace ConsoleApp {
    class RefOut {
        public static void run() {

            // ref 参数的变量必须先初始化
            var number = 10;
            Console.WriteLine(number); // 结果： 10
            AddNumber(ref number);
            Console.WriteLine(number); // 结果： 30

        }

        // ref
        static void AddNumber(ref int number) {
            number += 20;
        }
    }
}


namespace CsharpLog
{
    internal class Variable
    {
        public static void Run()
        {

            // 必须在定义时初始化
            // 可根据值自行判定变量类型
            // 只能用于局部变量

            var hello = "Hello";
            string world = "World";
            Console.WriteLine(hello + "," + world);
            Console.WriteLine("{0},{1}", hello, world);
            // 结果： Hello, World

            //int a; // 最好初始化并赋值
            long a = 100;
            var b = "中文";
            var c = true; // 推断为 bool 类型
            var d = 1.1;  // 推断为 double 类型
            var e = 2;    // 推断为 int 类型
            // 结果： 100, 中文, True, 1.1, 2
            Console.WriteLine("{0}, {1}, {2}, {3}, {4}", a, b, c, d, e);
        }
    }
}

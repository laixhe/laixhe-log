using System.Text;

namespace CsharpLog
{
    internal class StringNew
    {
        public static void Run()
        {
            // 拼接字符串
            var hello = "Hello";
            string world = "World";
            var strInt = 100;

            var hello0 = hello + "," + world + "," + strInt;
            var hello1 = string.Format("{0},{1},{2}", hello, world, strInt);
            var hello2 = $"{hello},{world},{strInt}";
            Console.WriteLine(hello0);
            Console.WriteLine(hello1);
            Console.WriteLine(hello2);
            // 结果： Hello,World,100
            // C#6.0 出现了 $ 方式拼接字符串，其实简单说就是 string.Format 简化操作版

            // 拼接字符串(其他)
            StringBuilder hello4 = new StringBuilder(50); // 初始化长度(预估)
            hello4.Append(hello);
            hello4.Append(',');
            hello4.Append(world);
            hello4.Append(',');
            hello4.Append(strInt.ToString());
            var hello5 = hello4.ToString();
            Console.WriteLine("[{0}]", hello5);
            // 结果： Hello,World,100
            // 当拼接的字符串多时，StringBuilder 的优势越来越明显

            // 字符串转成数字
            // 分隔字符串
            // 拼接字符串(数组)

        }
    }
}

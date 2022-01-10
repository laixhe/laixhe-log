// List(列表)、Dict(字典)

namespace CsharpLog {
    internal class ArrayListDict {
        public static void Run() {

            // 数组
            // 索引从 0 开始

            // 定义数组并初始化数据都为 0
            int[] ints1 = new int[6];
            foreach (int i in ints1) {
                //Console.WriteLine(i);
            }
            // 结果: 0 0 0 0 0 0

            // 定义数组并初始化
            int[] ints2 = new int[]{1,2,3};
            foreach (int i in ints2) {
                //Console.WriteLine(i);
            }
            // 结果:1 2 3

            // 获取某个值(没有这个 索引 时会报错)
            Console.WriteLine("array get={0}", ints2[2]);

            Console.WriteLine("=================================");

            // List(列表)
            // 索引从 0 开始

            var stringList = new List<string>();
            // 将数据新增到集合结尾处
            stringList.Add("123");
            stringList.Add("456");
            foreach(var item in stringList) {
                Console.WriteLine(item);
            }
            Console.WriteLine(stringList.ToString());


            // 修改指定索引的数据
            stringList[1] = "789";
            Console.WriteLine("-------------");
            foreach(var item in stringList) {
                Console.WriteLine(item);
            }
            // 移除指定索引的数据
            stringList.RemoveAt(0);
            // 移除内容为 xxx 的数据
            stringList.Remove("789");
            Console.WriteLine("-------------");
            foreach(var item in stringList) {
                Console.WriteLine(item);
            }
            // 再指定索引处插入数据
            stringList.Insert(0, "Hello World!");

            Console.WriteLine("-------------");
            foreach(var item in stringList) {
                Console.WriteLine(item);
            }

            // dict(字典)以 键值对 key:value
            var dict = new Dictionary<int, string>();
            dict.Add(1, "A");
            dict.Add(2, "B");
            dict.Add(5, "c");
            dict.Add(6, "d");
            Console.WriteLine("-------------");
            foreach(var item in dict) {
                Console.WriteLine("key={0} value={1}", item.Key, item.Value);
            }

        }
    }
}
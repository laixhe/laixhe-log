#### List(列表)、Dict(字典)
> 命名空间 System.Collections.Generic

```
using System;
using System.Collections.Generic;
namespace ConsoleApp {
    class Program {
        static void Main(string[] args) {
            //var ints = new int[6];
            //int[] ints = new int[]{1,2,3};
            
            // List(列表)
            // 索引从 0 开始
            var stringList = new List<string>();
            // 将数据新增到集合结尾处
            stringList.Add("123");
            stringList.Add("456");
            foreach(var item in stringList) {
                Console.WriteLine(item);
            }
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
```

#### 由关键字```const```声明常量，由```enum```枚举类型有数字枚举

```
using System;

namespace ConsoleApp {
    class Program {
        static void Main(string[] args)
        {
            var read = Color.Read;
            Console.WriteLine("read={0}", read);
            // 一开始就得赋值
            const int uid = 10;
            //uid = 11; // error
        }
    }
    // 枚举
    // 从 0 开始为元素编号
    // 可手动指定原始值
    enum Color {
        Read,
        Green,
        Blue
    }
}
```
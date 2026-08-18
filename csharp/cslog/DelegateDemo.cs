// 委托与 Lambda：delegate / Action / Func / 闭包 / 多播 / 事件
// 对应 Go golog function_test.go（函数值/闭包）、Python functions.py、C++ StdCallback

public static class DelegateDemo
{
    // 自定义委托类型：签名 = (int, int) -> int（对应 Go 函数类型 / C 函数指针）
    private delegate int BinaryOp(int a, int b);

    // 事件：基于委托的通知机制（对应观察者模式 / C# 事件）
    private static event Action<string>? MessagePublished;

    public static void Run()
    {
        // ===== 1. 委托声明与赋值 =====
        Console.WriteLine("--- delegate ---");

        BinaryOp add = (a, b) => a + b;            // Lambda 赋值
        BinaryOp mul = static (a, b) => a * b;     // static lambda：不捕获变量
        Console.WriteLine($"add(3,4) = {add(3, 4)}，mul(3,4) = {mul(3, 4)}"); // add(3,4) = 7，mul(3,4) = 12

        BinaryOp sub = Subtract;                   // 方法组转委托
        Console.WriteLine($"sub(7,2) = {sub(7, 2)}"); // sub(7,2) = 5

        // ===== 2. 内置委托 Action / Func（对应 Go 一等函数 / Python lambda）=====
        Console.WriteLine("--- Action / Func ---");

        Action<string> log = msg => Console.WriteLine($"  [log] {msg}"); // 无返回值
        log("Action 无返回值");

        Func<int, int, int> calc = (a, b) => a * b + 1;                 // 有返回值
        Console.WriteLine($"Func 计算 = {calc(3, 4)}"); // Func 计算 = 13

        Func<int, bool> isEven = n => n % 2 == 0;   // 谓词
        Console.WriteLine($"Func 谓词: 4 是偶数 = {isEven(4)}"); // Func 谓词: 4 是偶数 = True

        // 高阶函数：委托作为参数传入（LINQ 正是依赖委托）
        int[] nums = [1, 2, 3, 4, 5];
        Console.WriteLine($"Select 翻倍 = {string.Join(", ", nums.Select(x => x * 2))}"); // Select 翻倍 = 2, 4, 6, 8, 10
        Console.WriteLine($"Where 偶数 = {string.Join(", ", nums.Where(isEven))}"); // Where 偶数 = 2, 4

        // ===== 3. 闭包捕获（对应 Go 闭包 / Python closure）=====
        Console.WriteLine("--- 闭包 ---");

        int factor = 10;
        Func<int, int> scale = x => x * factor;  // 捕获外部变量 factor
        Console.WriteLine($"scale(5) = {scale(5)}"); // scale(5) = 50
        factor = 100;                            // 闭包引用的是变量本身而非快照
        Console.WriteLine($"修改 factor 后 scale(5) = {scale(5)}"); // 修改 factor 后 scale(5) = 500

        // 计数器闭包（对应 Go 闭包计数器）
        Func<int> counter = MakeCounter();
        Console.WriteLine($"counter = {counter()} {counter()} {counter()}"); // counter = 1 2 3

        // ===== 4. 多播委托：+= 追加，依次调用 =====
        Console.WriteLine("--- 多播委托 ---");

        Action announce = () => Console.WriteLine("  part1");
        announce += () => Console.WriteLine("  part2"); // 追加调用
        announce();                                    // 依次执行 part1、part2

        // ===== 5. 事件：+= 订阅 / -= 退订 =====
        Console.WriteLine("--- 事件 ---");

        Action<string> handlerA = msg => Console.WriteLine($"  订阅者A收到: {msg}");
        Action<string> handlerB = msg => Console.WriteLine($"  订阅者B收到: {msg}");
        MessagePublished += handlerA;
        MessagePublished += handlerB;
        MessagePublished?.Invoke("hello event"); // 触发事件

        MessagePublished -= handlerA;            // 退订订阅者A
        MessagePublished?.Invoke("after unsubscribe");
    }

    private static int Subtract(int a, int b) => a - b;

    // 返回闭包的函数（对应 Go 返回闭包 / Python 闭包工厂）
    private static Func<int> MakeCounter()
    {
        int count = 0;
        return () => ++count;
    }
}

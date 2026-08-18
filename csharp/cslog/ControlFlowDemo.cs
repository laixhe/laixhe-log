// 控制流：条件分支 / switch / 循环 / 跳转语句
// 对应 Go golog control_flow_test.go、Python pylog control_flow.py

public static class ControlFlowDemo
{
    public static void Run()
    {
        // ===== 1. 条件分支（对应 Go if / Java if-else）=====
        Console.WriteLine("--- if / else ---");

        int score = 85;
        string grade = score >= 90 ? "A" : score >= 80 ? "B" : score >= 60 ? "C" : "D";
        Console.WriteLine($"score={score} grade={grade}"); // score=85 grade=B

        int hour = 10;
        string greeting = hour < 12 ? "上午好" : hour < 18 ? "下午好" : "晚上好";
        Console.WriteLine($"hour={hour} -> {greeting}"); // hour=10 -> 上午好

        // ===== 2. switch 表达式（C# 8+，对应 Java switch 表达式 / Go switch）=====
        Console.WriteLine("--- switch ---");

        string day = "mon";
        string dayName = day switch
        {
            "mon" => "星期一",
            "fri" => "星期五",
            _ => "其它" // 默认分支（对应 default）
        };
        Console.WriteLine($"{day} -> {dayName}"); // mon -> 星期一

        // 带 when 守卫的 switch（对应 Python match-case 的 if 条件）
        int n = 2;
        string parity = n switch
        {
            0 => "zero",
            _ when n % 2 == 0 => "even",
            _ => "odd"
        };
        Console.WriteLine($"n={n} -> {parity}"); // n=2 -> even

        // ===== 3. 循环（对应 Go for / Python for-in）=====
        Console.WriteLine("--- 循环 ---");

        // for
        int sum = 0;
        for (int i = 1; i <= 10; i++) sum += i;
        Console.WriteLine($"1..10 求和 = {sum}"); // 1..10 求和 = 55

        // foreach（对应 Go for range / Python for-in）
        string[] fruits = ["apple", "banana", "cherry"];
        foreach (string f in fruits) Console.Write($"{f} ");
        Console.WriteLine();

        // 带索引的 foreach（对应 Python enumerate / Go 手动索引）
        foreach (var (i, f) in fruits.Select((v, i) => (i, v)))
        {
            Console.WriteLine($"  [{i}] = {f}");
        }

        // while / do-while（do-while 至少执行一次）
        int k = 5, cnt = 0;
        while (k > 0) { k -= 2; cnt++; }
        Console.WriteLine($"while 循环次数 = {cnt}"); // while 循环次数 = 3

        int m = 0;
        do { m++; } while (m < 3);
        Console.WriteLine($"do-while 执行后 m = {m}"); // do-while 执行后 m = 3

        // ===== 4. 跳转语句（break / continue / return）=====
        Console.WriteLine("--- break / continue ---");

        // break：找到第一个质数提前退出
        int firstPrime = 0;
        for (int i = 2; i < 100; i++)
        {
            bool isPrime = true;
            for (int j = 2; j * j <= i; j++)
            {
                if (i % j == 0) { isPrime = false; break; }
            }
            if (isPrime) { firstPrime = i; break; }
        }
        Console.WriteLine($"100 以内第一个质数 = {firstPrime}"); // 100 以内第一个质数 = 2

        // continue：跳过奇数只收集偶数
        var evens = new List<int>();
        for (int i = 1; i <= 10; i++)
        {
            if (i % 2 != 0) continue;
            evens.Add(i);
        }
        Console.WriteLine($"1..10 的偶数 = {string.Join(", ", evens)}"); // 1..10 的偶数 = 2, 4, 6, 8, 10
    }
}

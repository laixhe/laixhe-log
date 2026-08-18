// 随机数：Random / 区间 / 洗牌 / 抽样 / 种子 / 随机字符串
// 对应 C++ cppapp StdRandom、Python random、Go math/rand
// 注意：随机输出无需对照预期值；唯一确定的是"种子"小节同种子产生相同序列

public static class RandomDemo
{
    public static void Run()
    {
        // ===== 1. 基本随机数（对应 Python random.random / Go rand）=====
        Console.WriteLine("--- 基本随机 ---");

        Random rng = new(); // .NET 6+ 无参构造：自动种子、线程安全
        Console.WriteLine($"Next() = {rng.Next()}");
        Console.WriteLine($"Next(1, 101) = {rng.Next(1, 101)}");    // [1, 100] 区间整数
        Console.WriteLine($"NextDouble() = {rng.NextDouble():F4}"); // [0, 1) 浮点

        // ===== 2. 常用模拟 =====
        Console.WriteLine("--- 常用模拟 ---");

        Console.WriteLine($"掷骰子 = {rng.Next(1, 7)}");       // 1..6
        Console.WriteLine($"随机布尔 = {rng.Next(2) == 1}");   // 掷硬币

        // ===== 3. 洗牌与抽样（对应 Python random.shuffle / random.sample）=====
        Console.WriteLine("--- 洗牌与抽样 ---");

        // Fisher-Yates 洗牌
        List<string> deck = ["A", "B", "C", "D", "E"];
        for (int i = deck.Count - 1; i > 0; i--)
        {
            int j = rng.Next(i + 1);
            (deck[i], deck[j]) = (deck[j], deck[i]);
        }
        Console.WriteLine($"洗牌后 = {string.Join(" ", deck)}");

        // 抽样：从集合随机取 3 个（OrderBy + Take）
        int[] pool = [10, 20, 30, 40, 50];
        Console.WriteLine($"抽 3 个 = {string.Join(", ", pool.OrderBy(_ => rng.Next()).Take(3))}");

        // ===== 4. 确定性种子（对应 Python random.seed / Go rand.NewSource）=====
        Console.WriteLine("--- 种子 ---");

        var r1 = new Random(42);
        var r2 = new Random(42);
        Console.WriteLine($"同种子 r1 = {r1.Next(100)}，r2 = {r2.Next(100)}（相同种子产生相同序列，便于复现）");

        // ===== 5. 随机字符串（验证码 / token 场景）=====
        Console.WriteLine("--- 随机字符串 ---");

        const string chars = "abcdefghijklmnopqrstuvwxyz0123456789";
        string token = new string(Enumerable.Repeat(chars, 8)
            .Select(s => s[rng.Next(s.Length)]).ToArray());
        Console.WriteLine($"随机 8 位 token = {token}");

        // ===== 6. 全局共享实例 Random.Shared（线程安全）=====
        Console.WriteLine($"Random.Shared.NextDouble() = {Random.Shared.NextDouble():F4}");
    }
}

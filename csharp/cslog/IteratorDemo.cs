// 迭代器与 LINQ：Select / Where / Take / Drop / Zip / Aggregate / Any / All / 分组实战
// 对应 Rust rustlog iterators.rs、Go golog slice 迭代、Python iterators.py

public static class IteratorDemo
{
    public static void Run()
    {
        // ===== 1. map / filter（对应 Rust map/filter、TS map/filter）=====
        Console.WriteLine("--- map / filter ---");

        // 1..10 的平方
        var squares = Enumerable.Range(1, 10).Select(n => n * n);
        Console.WriteLine($"1..10 平方: {string.Join(" ", squares)}");

        // 过滤长度 ≤ 3 的单词
        string[] words = { "go", "c++", "javascript", "rust", "java", "js" };
        var shortWords = words.Where(w => w.Length <= 3);
        Console.WriteLine($"长度≤3 的单词: {string.Join(" ", shortWords)}");

        // filter_map：Select + Where 组合，选出合法数字（对应 Rust filter_map）
        string[] maybeNumbers = { "123", "abc", "456", "78x", "789" };
        var validNumbers = maybeNumbers
            .Select(s => int.TryParse(s, out int n) ? (int?)n : null)
            .Where(n => n.HasValue)
            .Select(n => n!.Value);
        Console.WriteLine($"filter_map 选出合法数字: {string.Join(" ", validNumbers)}");

        // ===== 2. take / drop（对应 Rust take/drop、TS slice）=====
        Console.WriteLine("--- take / drop ---");

        var r1 = Enumerable.Range(1, 10);
        Console.WriteLine($"take(3): {string.Join(" ", r1.Take(3))}");
        Console.WriteLine($"skip(7): {string.Join(" ", r1.Skip(7))}"); // drop 前 7 个

        // ===== 3. zip / 展平（对应 Rust zip、TS flatMap）=====
        Console.WriteLine("--- zip / 展平 ---");

        string[] names = { "Alice", "Bob", "Charlie" };
        int[] scores = { 95, 87, 92 };
        var zipped = names.Zip(scores, (name, score) => $"({name},{score})");
        Console.WriteLine($"zip 配对: {string.Join(" ", zipped)}");

        int[][] nested = { new[] { 1, 2 }, new[] { 3, 4 }, new[] { 5, 6 } };
        Console.WriteLine($"SelectMany 展平: {string.Join(" ", nested.SelectMany(x => x))}");

        // ===== 4. reduce / any / all（对应 Rust fold、TS reduce）=====
        Console.WriteLine("--- reduce / any / all ---");

        Console.WriteLine($"Aggregate 1..10 = {Enumerable.Range(1, 10).Aggregate(0, (acc, n) => acc + n)}");
        Console.WriteLine($"any > 10? {Enumerable.Range(1, 10).Any(n => n > 10)}");
        Console.WriteLine($"all > 0?  {Enumerable.Range(1, 10).All(n => n > 0)}");

        // ===== 5. partition（对应 Rust partition：拆成满足/不满足两组）=====
        Console.WriteLine("--- partition ---");

        int[] r2 = { 3, 1, 4, 1, 5, 9, 2, 6 };
        var evens = r2.Where(n => n % 2 == 0).ToList();
        var odds = r2.Where(n => n % 2 != 0).ToList();
        Console.WriteLine($"偶={string.Join(" ", evens)}  奇={string.Join(" ", odds)}");

        // ===== 6. 综合实战：R&D 部门 30+ 员工平均月薪（对应 Go/Rust/TS 综合示例）=====
        Console.WriteLine("--- 综合实战：平均月薪 ---");

        var staff = new (string Name, string Dept, int Age, decimal Salary)[]
        {
            ("Alice", "R&D", 28, 45000m),
            ("Bob", "R&D", 35, 55000m),
            ("Charlie", "R&D", 32, 50000m),
            ("David", "HR", 40, 30000m),
            ("Eve", "R&D", 25, 35000m),
        };
        // 链式：过滤 → 提取月薪 → 求平均
        decimal avg = staff
            .Where(p => p.Dept == "R&D" && p.Age >= 30)
            .Select(p => p.Salary)
            .DefaultIfEmpty() // 空序列时给默认值，避免 Average 抛异常
            .Average();
        Console.WriteLine($"R&D 30+ 员工平均月薪: {avg:N0} 元/月");
    }
}

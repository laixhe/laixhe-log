// 性能优化：LINQ 写法效率对比 / 数据结构选型 / 字符串拼接 / 预分配容量
// 对应 Go benchmark 基准测试、Rust criterion、C++ 性能优化手法
// 注：dotnet run 默认 Debug 模式；相对趋势不变，Release 模式差异更明显（可用 dotnet run -c Release 对比）
using System.Diagnostics;
using System.Text;

public static class PerfDemo
{
    private const int N = 1_000_000;

    public static void Run()
    {
        var nums = Enumerable.Range(1, N).ToArray();

        // ===== 1. LINQ 写法效率 =====
        Console.WriteLine("--- LINQ 写法效率 ---");

        // Any() 短路：找到第一个即返回 true；Count()>0 要数完整个序列
        var range = nums.Where(n => true); // 惰性序列（Where 结果无 Count 快捷优化）
        Time("Count() > 0 判断非空", () => range.Count() > 0);
        Time("Any() 判断非空", () => range.Any());

        // First(p) 直接匹配 vs Where(p).First()：后者多包一层迭代器
        Time("Where(p).First() 双层", () => nums.Where(n => n == N - 1).First());
        Time("First(p) 单层匹配", () => nums.First(n => n == N - 1));

        // Count(p) 内建计数 vs Where(p).Count()：语义相同，前者少一次委托分发
        Time("Where(p).Count()", () => nums.Where(n => n % 2 == 0).Count());
        Time("Count(p) 直接计数", () => nums.Count(n => n % 2 == 0));

        // 过滤优先：Where 前置只对一半元素执行 Select；先映射则全部元素都执行
        Time("Where→Select 过滤优先", () => nums.Where(n => n % 2 == 0).Select(n => n * 2L).Sum());
        Time("Select→Where 先映射", () => nums.Select(n => n * 2L).Where(n => n % 2 == 0).Sum());

        // 惰性序列重复遍历：每次聚合都重跑谓词；ToList 物化后谓词只跑一次
        // 谓词较重（字符串解析）时差距更明显
        var rows = Enumerable.Range(1, 200_000).Select(i => $"id_{i}").ToArray();
        Time("IEnumerable 重复遍历×2", () =>
        {
            var q = rows.Where(r => int.Parse(r.AsSpan(3)) % 100 == 0);
            return q.Count() + q.Sum(r => r.Length);
        });
        Time("ToList 物化后遍历", () =>
        {
            var m = rows.Where(r => int.Parse(r.AsSpan(3)) % 100 == 0).ToList();
            return m.Count + m.Sum(r => r.Length);
        });

        // ===== 2. 数据结构选型 =====
        Console.WriteLine("--- 数据结构选型 ---");

        // HashSet 查找 O(1) vs List 线性查找 O(n)（重复 5 次放大单次耗时差）
        var list = Enumerable.Range(1, N).ToList();
        var set = list.ToHashSet();
        Time("List.Contains 线性查找×5", () =>
        {
            bool r = true;
            for (int i = 0; i < 5; i++) r &= list.Contains(N + i); // N+i 均不存在，每次全扫
            return r;
        });
        Time("HashSet.Contains O(1)×5", () =>
        {
            bool r = true;
            for (int i = 0; i < 5; i++) r &= set.Contains(N + i);
            return r;
        });

        // Dictionary 索引 O(1) vs List 线性查找 O(n)
        var dict = Enumerable.Range(1, N).ToDictionary(i => i, i => i * 2);
        var pairs = dict.ToList();
        Time("List.First 线性查找", () => pairs.First(p => p.Key == N).Value);
        Time("Dictionary 索引 O(1)", () => dict[N]);

        // TryGetValue 单次哈希 vs ContainsKey+索引器 两次哈希（×1M 放大差距）
        Time("ContainsKey + 索引器×1M", () =>
        {
            long s = 0;
            for (int i = 1; i <= N; i++) if (dict.ContainsKey(i)) s += dict[i];
            return s;
        });
        Time("TryGetValue 单次查找×1M", () =>
        {
            long s = 0;
            for (int i = 1; i <= N; i++) if (dict.TryGetValue(i, out var v)) s += v;
            return s;
        });

        // ===== 3. 其他性能技巧 =====
        Console.WriteLine("--- 其他技巧 ---");

        // 预分配容量：避免扩容时数组搬运（对应 C++ vector::reserve）
        Time("List 默认扩容添加", () =>
        {
            var l = new List<int>();
            for (int i = 0; i < N; i++) l.Add(i);
            return l.Count;
        });
        Time("List 预分配容量添加", () =>
        {
            var l = new List<int>(N);
            for (int i = 0; i < N; i++) l.Add(i);
            return l.Count;
        });

        // 字符串拼接：+= 每次创建新字符串（O(n²)），StringBuilder 原地缓冲
        Time("字符串 += 拼接", () =>
        {
            string s = "";
            for (int i = 0; i < 20_000; i++) s += i;
            return s.Length;
        });
        Time("StringBuilder 拼接", () =>
        {
            var sb = new StringBuilder();
            for (int i = 0; i < 20_000; i++) sb.Append(i);
            return sb.Length;
        });

        // 排序选型：LINQ OrderBy 带委托与额外分配；Array.Sort 原地快排
        var random = new Random(42);
        var unsorted = Enumerable.Range(1, 100_000).OrderBy(_ => random.Next()).ToArray();
        Time("LINQ OrderBy 排序", () =>
        {
            var t = unsorted.OrderBy(x => x).ToArray();
            return t.Length;
        });
        Time("Array.Sort 原地排序", () =>
        {
            var copy = (int[])unsorted.Clone();
            Array.Sort(copy);
            return copy.Length;
        });
    }

    // 预热 + 计时：重复 reps 次取总耗时（相对比较看趋势即可）
    private static void Time(string label, Func<object> action, int reps = 3)
    {
        action();     // 预热：触发 JIT 编译，排除首次编译耗时干扰
        GC.Collect(); // 减少上一轮 GC 残留干扰
        var sw = Stopwatch.StartNew();
        for (int i = 0; i < reps; i++) _ = action();
        sw.Stop();
        Console.WriteLine($"  {label,-26}: {sw.ElapsedMilliseconds,5} ms");
    }
}

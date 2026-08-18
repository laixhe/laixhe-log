// LINQ 进阶：分组 / 连接 / 排序 / 集合运算 / 元素操作 / 延迟执行 / 查询语法 / 分页
// 对应 SQL GROUP BY / JOIN 思维、Java Stream 进阶、Python itertools

public static class IteratorAdvDemo
{
    private record Emp(string Name, string Dept, int Age, decimal Salary);

    public static void Run()
    {
        var emps = new List<Emp>
        {
            new("Alice", "R&D", 28, 45000m),
            new("Bob", "R&D", 35, 55000m),
            new("Charlie", "R&D", 32, 50000m),
            new("David", "HR", 40, 30000m),
            new("Eve", "HR", 25, 32000m),
        };

        // ===== 1. 分组聚合 GroupBy（对应 SQL GROUP BY）=====
        Console.WriteLine("--- GroupBy ---");

        foreach (var g in emps.GroupBy(e => e.Dept))
        {
            Console.WriteLine($"  {g.Key}: {g.Count()} 人，平均 {g.Average(e => e.Salary):N0} 元"); // R&D: 3 人，平均 50,000 元 / HR: 2 人，平均 31,000 元
        }

        // ===== 2. 连接 Join（对应 SQL INNER JOIN / LEFT JOIN）=====
        Console.WriteLine("--- Join ---");

        var depts = new[]
        {
            new { Code = "R&D", Name = "研发部" },
            new { Code = "HR", Name = "人力资源部" },
            new { Code = "FIN", Name = "财务部" }, // 没有员工
        };
        var inner = emps.Join(depts, e => e.Dept, d => d.Code, (e, d) => $"{e.Name} 属于 {d.Name}");
        Console.WriteLine($"内连接: {string.Join(" | ", inner)}"); // 内连接: Alice 属于 研发部 | Bob 属于 研发部 | ... | Eve 属于 人力资源部

        // 左连接：GroupJoin + SelectMany + DefaultIfEmpty
        var left = depts.GroupJoin(emps, d => d.Code, e => e.Dept, (d, es) => $"{d.Name}（{es.Count()} 人）");
        Console.WriteLine($"左连接: {string.Join(" | ", left)}"); // 左连接: 研发部（3 人） | 人力资源部（2 人） | 财务部（0 人）

        // ===== 3. 排序 OrderBy / ThenBy / MaxBy =====
        Console.WriteLine("--- 排序 ---");

        var sorted = emps.OrderBy(e => e.Dept).ThenByDescending(e => e.Salary)
            .Select(e => $"{e.Dept}/{e.Name}({e.Salary:N0})");
        Console.WriteLine($"OrderBy 部门 + ThenByDescending 薪水: {string.Join(" ", sorted)}"); // HR/Eve(32,000) HR/David(30,000) R&D/Bob(55,000) ...

        // MaxBy / MinBy：直接取极值元素（.NET 6+）
        Console.WriteLine($"MaxBy 最高薪 = {emps.MaxBy(e => e.Salary)!.Name}，MinBy 最低薪 = {emps.MinBy(e => e.Salary)!.Name}"); // MaxBy 最高薪 = Bob，MinBy 最低薪 = David

        // ===== 4. 集合运算 Union / Intersect / Except =====
        Console.WriteLine("--- 集合运算 ---");

        int[] a = [1, 2, 3, 4];
        int[] b = [3, 4, 5, 6];
        Console.WriteLine($"并集: {string.Join(" ", a.Union(b))}"); // 并集: 1 2 3 4 5 6
        Console.WriteLine($"交集: {string.Join(" ", a.Intersect(b))}"); // 交集: 3 4
        Console.WriteLine($"差集 A-B: {string.Join(" ", a.Except(b))}"); // 差集 A-B: 1 2

        // DistinctBy / Chunk（.NET 6+）
        string[] words = ["apple", "banana", "avocado", "cherry"];
        Console.WriteLine($"DistinctBy 首字母: {string.Join(" ", words.DistinctBy(w => w[0]))}"); // DistinctBy 首字母: apple banana cherry
        Console.WriteLine($"Chunk(2): {string.Join(" | ", Enumerable.Range(1, 5).Chunk(2).Select(c => $"[{string.Join(",", c)}]"))}"); // Chunk(2): [1,2] | [3,4] | [5]

        // ===== 5. 元素操作 First / Single / Last =====
        Console.WriteLine("--- 元素操作 ---");

        Console.WriteLine($"First = {emps.First().Name}，Last = {emps.Last().Name}"); // First = Alice，Last = Eve
        Console.WriteLine($"FirstOrDefault（无匹配返回 null）: {emps.FirstOrDefault(e => e.Age > 100)?.Name ?? "无"}"); // FirstOrDefault（无匹配返回 null）: 无
        Console.WriteLine($"SingleOrDefault（唯一匹配）: {emps.SingleOrDefault(e => e.Name == "Alice")?.Name}"); // SingleOrDefault（唯一匹配）: Alice
        Console.WriteLine($"ElementAt(2) = {emps.ElementAt(2).Name}"); // ElementAt(2) = Charlie

        // ===== 6. 延迟执行（Deferred Execution）=====
        Console.WriteLine("--- 延迟执行 ---");

        var nums = new List<int> { 1, 2, 3 };
        var query = nums.Where(n => n > 1); // 只是描述，尚未遍历
        nums.Add(4);                        // 真正枚举时才读当前数据
        Console.WriteLine($"延迟执行: {string.Join(" ", query)}"); // 延迟执行: 2 3 4

        var snapshot = nums.Where(n => n > 1).ToList(); // ToList 立即执行并快照
        nums.Add(5);
        Console.WriteLine($"ToList 快照: {string.Join(" ", snapshot)}（之后加 5 不影响）"); // ToList 快照: 2 3 4

        // ===== 7. 查询语法（from...select）vs 方法语法 =====
        Console.WriteLine("--- 查询语法 ---");

        var qs = from e in emps
                 where e.Age >= 30
                 orderby e.Salary descending
                 select $"{e.Name}({e.Salary:N0})";
        Console.WriteLine($"查询语法: {string.Join(" ", qs)}"); // 查询语法: Bob(55,000) Charlie(50,000) David(30,000)

        // ===== 8. 分页实战（Skip / Take）=====
        Console.WriteLine("--- 分页 ---");

        int pageSize = 2, page = 1;
        var page2 = emps.OrderBy(e => e.Name).Skip((page - 1) * pageSize).Take(pageSize);
        Console.WriteLine($"第 {page + 1} 页（每页 {pageSize}）: {string.Join(", ", page2.Select(e => e.Name))}"); // 第 2 页（每页 2）: Alice, Bob
    }
}

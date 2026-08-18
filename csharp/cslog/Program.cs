// cslog：C# / .NET 标准库学习示例（参考 Go golog / Rust rustlog 主题）
// 运行全部: dotnet run
// 运行单个: dotnet run -- Number   （模块名或文件名均可，如 ControlFlow / CollectionDemo / Http）

// 模块过滤 + 单模块容错：指定单个模块只运行它；某个模块异常不会中断其余模块
string? only = args.Length > 0 ? args[0] : null;

static bool Match(string key, string? only) =>
    only is not null &&
    (string.Equals(only, key, StringComparison.OrdinalIgnoreCase) ||
     string.Equals(only, key + "Demo", StringComparison.OrdinalIgnoreCase));

void Run(string key, Action run)
{
    if (only is not null && !Match(key, only)) return;
    try { run(); }
    catch (Exception ex) { Console.WriteLine($"\n[{key} 运行失败] {ex.GetType().Name}: {ex.Message}"); }
}

Run("ControlFlow", ControlFlowDemo.Run);
Run("Class", ClassDemo.Run);
Run("Enum", EnumDemo.Run);
Run("Number", NumberDemo.Run);
Run("Random", RandomDemo.Run);
Run("Bits", BitsDemo.Run);
Run("String", StringDemo.Run);
Run("Nullable", NullableDemo.Run);
Run("Collection", CollectionDemo.Run);
Run("Iterator", IteratorDemo.Run);
Run("IteratorAdv", IteratorAdvDemo.Run);
Run("Time", TimeDemo.Run);
Run("Exception", ExceptionDemo.Run);
Run("File", FileDemo.Run);
Run("Regex", RegexDemo.Run);
Run("Generic", GenericDemo.Run);
Run("GenericAdv", GenericAdvDemo.Run);
Run("Delegate", DelegateDemo.Run);
Run("Perf", PerfDemo.Run);
Run("Sync", SyncDemo.Run);
Run("Json", JsonDemo.Run);
Run("Http", () => HttpDemo.RunAsync().GetAwaiter().GetResult());

Console.WriteLine("\n全部演示完成");
if (only is not null) Console.WriteLine($"（已按参数筛选：dotnet run -- {only}）");

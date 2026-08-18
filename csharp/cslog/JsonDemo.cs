// JSON 序列化 / 反序列化：System.Text.Json
// 对应 Go golog json_test.go（omitempty / string tag）、Java JsonDemo.java

using System.Text.Json;
using System.Text.Json.Serialization;

public static class JsonDemo
{
    public static void Run()
    {
        // ===== 1. 基础序列化 / 反序列化（对应 JsonSerializer）=====
        Console.WriteLine("--- 序列化 / 反序列化 ---");

        var user = new User { Name = "laixhe", Age = 18, Tags = new[] { "go", "rust" } };
        string json = JsonSerializer.Serialize(user);
        Console.WriteLine($"序列化: {json}");
        // {"Name":"laixhe","Age":18,"Tags":["go","rust"]}

        var parsed = JsonSerializer.Deserialize<User>(json)!;
        Console.WriteLine($"反序列化 name = {parsed.Name}, age = {parsed.Age}");

        // ===== 2. omitempty：空值忽略（对应 Go omitempty tag）=====
        Console.WriteLine("--- omitempty ---");

        var emptyJson = new User { Name = "ok", Tags = Array.Empty<string>() }; // Age 默认 0、Tags 空
        Console.WriteLine($"omitempty 后: {JsonSerializer.Serialize(emptyJson, JsonOpts.OmitEmpty)}");
        // {"Name":"ok","Tags":[]}  注意：数组 [] 在 System.Text.Json 不会省略，且默认策略为首字母大写

        // ===== 3. 属性命名策略：小驼峰（对应 Go json tag / Java @JsonProperty）=====
        Console.WriteLine("--- 命名策略 ---");

        Console.WriteLine($"camelCase: {JsonSerializer.Serialize(user, JsonOpts.CamelCase)}");
        // {"name":"laixhe","age":18,"tags":["go","rust"]}

        // ===== 4. 美化输出（对应 json.MarshalIndent）=====
        Console.WriteLine("--- 美化输出 ---");

        Console.WriteLine(JsonSerializer.Serialize(user, new JsonSerializerOptions { WriteIndented = true }));

        // ===== 5. 解析失败抛异常（对应 JSONDecodeError）=====
        Console.WriteLine("--- 解析失败 ---");

        try
        {
            JsonSerializer.Deserialize<Dictionary<string, object>>("{invalid json}");
        }
        catch (JsonException e)
        {
            Console.WriteLine($"解析失败: {e.Message}");
        }

        // ===== 6. 动态 JSON（对应 Go map[string]any / TS JSON.parse）=====
        Console.WriteLine("--- 动态 JSON ---");

        // 注意：Dictionary<string, object> 反序列化时值实际是 JsonElement，打印显示原始 JSON 文本
        var dict = JsonSerializer.Deserialize<Dictionary<string, object>>("""{"path":"/index/index","age":"18","score":"88.8","is_pass":"false"}""")!;
        Console.WriteLine($"path = {dict["path"]}, age = {dict["age"]}, is_pass = {dict["is_pass"]}");

        // 需要真实类型时，用 JsonDocument / JsonElement 显式读取并转换
        using var doc = JsonDocument.Parse("""{"age":18,"is_pass":false}""");
        JsonElement root = doc.RootElement;
        Console.WriteLine($"JsonElement: age={root.GetProperty("age").GetInt32()} is_pass={root.GetProperty("is_pass").GetBoolean()}");
    }

    // 演示模型：带 JsonIgnore 条件实现类似 Go omitempty 的效果
    private class User
    {
        public string? Name { get; set; }

        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public int Age { get; set; }

        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
        public string[]? Tags { get; set; }
    }

    // 常用序列化选项
    private static class JsonOpts
    {
        public static readonly JsonSerializerOptions OmitEmpty = new()
        {
            DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingDefault,
        };

        public static readonly JsonSerializerOptions CamelCase = new()
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        };
    }
}

// HTTP 客户端 + 服务端：HttpClient / HttpListener
// 对应 Go golog http_client_test.go + http_serve_test.go

using System.Net;
using System.Net.Http.Json;

public static class HttpDemo
{
    public static async Task RunAsync()
    {
        // 端口可通过环境变量 CSLOG_HTTP_PORT 覆盖（默认 18080），避免端口被占用时启动失败
        int port = int.TryParse(Environment.GetEnvironmentVariable("CSLOG_HTTP_PORT"), out int p) ? p : 18080;
        string baseUrl = $"http://127.0.0.1:{port}/";

        // ===== 1. 本地起一个 HTTP 服务端（对应 Go ListenAndServe）=====
        Console.WriteLine("--- HTTP 服务端 ---");

        var listener = new HttpListener();
        listener.Prefixes.Add(baseUrl);
        listener.Start();
        Console.WriteLine($"服务已启动: {baseUrl}");

        _ = Task.Run(() =>
        {
            while (listener.IsListening)
            {
                HttpListenerContext ctx;
                try { ctx = listener.GetContext(); }
                catch { break; }
                _ = Task.Run(async () => await HandleRequest(ctx));
            }
        });

        // ===== 2. HTTP 客户端 GET（对应 Go http.Get）=====
        Console.WriteLine("--- GET 请求 ---");

        using var client = new HttpClient { BaseAddress = new Uri(baseUrl) };
        string getBody = await client.GetStringAsync("/hello?name=laixhe");
        Console.WriteLine($"GET /hello → {getBody}");

        // ===== 3. HTTP 客户端 POST + JSON（对应 Go client.Post）=====
        Console.WriteLine("--- POST JSON ---");

        var resp = await client.PostAsJsonAsync("/echo", new { name = "laixhe", age = 18 });
        string postBody = await resp.Content.ReadAsStringAsync();
        Console.WriteLine($"POST /echo → {postBody}");

        listener.Stop();
    }

    // 处理请求：GET /hello 返回文本，POST /echo 回显 JSON
    private static async Task HandleRequest(HttpListenerContext ctx)
    {
        string path = ctx.Request.Url!.AbsolutePath;
        try
        {
            if (path == "/hello")
            {
                string name = ctx.Request.QueryString["name"] ?? "world";
                byte[] body = System.Text.Encoding.UTF8.GetBytes($"hello, {name}!");
                ctx.Response.StatusCode = 200;
                ctx.Response.ContentType = "text/plain; charset=utf-8";
                ctx.Response.ContentLength64 = body.Length;
                await ctx.Response.OutputStream.WriteAsync(body);
            }
            else if (path == "/echo")
            {
                string body = await new StreamReader(ctx.Request.InputStream).ReadToEndAsync();
                byte[] bytes = System.Text.Encoding.UTF8.GetBytes(body);
                ctx.Response.StatusCode = 200;
                ctx.Response.ContentType = "application/json";
                ctx.Response.ContentLength64 = bytes.Length;
                await ctx.Response.OutputStream.WriteAsync(bytes);
            }
            else
            {
                ctx.Response.StatusCode = 404;
            }
        }
        finally
        {
            ctx.Response.Close();
        }
    }
}

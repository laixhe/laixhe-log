var builder = WebApplication.CreateBuilder(args);

// 注册 MVC 框架依赖项并发现控制器
builder.Services.AddControllers();

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

// 判断环境变量
if (app.Environment.IsDevelopment())
{
    // 异常处理中间件
    app.UseDeveloperExceptionPage();
    app.UseSwagger();
    app.UseSwaggerUI();
}

// 可访问和使用静态文件
//app.UseStaticFiles();
// 认证中间件
//app.UseAuthentication();
// 授权中间件
//app.UseAuthorization();
// 跨域中间件
//app.UseCors();
// 全局异常处理中间件
//app.UseExceptionHandler();
// 代理头信息转发中间件
//app.UseForwardedHeaders();
// Session中间件
//app.UseSession();
// 注册控制器路由和 MVC 中间件
app.MapControllers();

// HTTP请求和响应日志中间件
app.UseHttpLogging();

app.Run();

// Mini API
// var builder = WebApplication.CreateBuilder(args);
// var app = builder.Build();
// app.MapGet("/", () => "Hello GET");
// app.MapPost("/", () => "Hello POST");
// app.MapPut("/", () => "Hello PUT");
// app.MapDelete("/", () => "Hello DELETE");
// app.MapGet("/user/{name}", (string name) => $"Hello {name}");
// app.Run();

const std = @import("std");
const testing = std.testing;
const net = std.Io.net;

const PORT = 12345;

// 回显服务端：读取固定长度数据并原样返回（在独立线程运行）
fn echoServer() void {
    var threaded = std.Io.Threaded.init(std.heap.page_allocator, .{});
    defer threaded.deinit();
    const io = threaded.io();

    var addr = net.IpAddress.parse("127.0.0.1", PORT) catch unreachable;
    var server = net.IpAddress.listen(&addr, io, .{}) catch unreachable;
    defer server.deinit(io);

    var stream = server.accept(io) catch unreachable;
    defer stream.close(io);

    var rbuf: [1024]u8 = undefined;
    var reader = stream.reader(io, &rbuf);
    var wbuf: [1024]u8 = undefined;
    var writer = stream.writer(io, &wbuf);

    // 读取固定 5 字节并回显
    const data = reader.interface.readAlloc(std.heap.page_allocator, 5) catch return;
    defer std.heap.page_allocator.free(data);
    writer.interface.writeAll(data) catch {};
    writer.interface.flush() catch {}; // 刷新缓冲区，真正发送
}

test "TCP 客户端连接本地回显服务器" {
    // 启动服务端线程
    const thread = try std.Thread.spawn(.{}, echoServer, .{});

    // 客户端连接
    const io = testing.io;
    var addr = net.IpAddress.parse("127.0.0.1", PORT) catch unreachable;
    var stream = net.IpAddress.connect(&addr, io, .{ .mode = .stream }) catch unreachable;
    defer stream.close(io);

    // 发送数据
    var wbuf: [1024]u8 = undefined;
    var writer = stream.writer(io, &wbuf);
    try writer.interface.writeAll("hello");
    try writer.interface.flush(); // 刷新缓冲区，真正发送

    // 接收回显
    var rbuf: [1024]u8 = undefined;
    var reader = stream.reader(io, &rbuf);
    const recv = try reader.interface.readAlloc(testing.allocator, 5);
    defer testing.allocator.free(recv);

    try testing.expectEqualStrings("hello", recv);

    thread.join();
}

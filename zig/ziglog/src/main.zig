const std = @import("std");

// Zig 0.16 引入 "Juicy Main"：入口函数接收 std.process.Init 参数，
// 一次性获取 allocator、Io、命令行参数、环境变量等。
pub fn main(init: std.process.Init) !void {
    const io = init.io; // I/O 接口

    // 标准输出：传入空缓冲区（&.{}）即无缓冲，每次 print 直接写出；
    // 若传入有容量的缓冲区，则需在结束时调用 flush() 才会真正输出。
    var stdout_writer = std.Io.File.stdout().writer(io, &.{});
    const stdout = &stdout_writer.interface;

    try stdout.print("Hello, Zig!\n", .{});

    // 基础类型
    const answer: i32 = 42;          // 有符号 32 位整数
    const pi: f64 = 3.14159;         // 64 位浮点数
    try stdout.print("answer = {d}, pi = {d}\n", .{ answer, pi });

    // 数组与循环
    const items = [_]i32{ 1, 2, 3, 4, 5 };
    var sum: i32 = 0;
    for (items) |item| {
        sum += item;
    }
    try stdout.print("sum = {d}\n", .{sum});

    // 读取命令行参数
    const args = try init.minimal.args.toSlice(init.arena.allocator());
    try stdout.print("arg count = {d}\n", .{args.len});
}

const std = @import("std");
const testing = std.testing;

// Zig 0.16 的文件 I/O 统一在 std.Io.Dir / std.Io.File 下，
// 通过 io 参数注入。测试环境用 std.testing.io 和临时目录。

test "写入并读取文件" {
    const io = testing.io;
    const gpa = testing.allocator;

    // 创建临时目录，测试结束后自动清理
    var tmp = testing.tmpDir(.{});
    defer tmp.cleanup();

    // 写文件
    try tmp.dir.writeFile(io, .{
        .sub_path = "hello.txt",
        .data = "Hello, Zig!",
    });

    // 读文件（返回的 buffer 需要手动释放）
    const content = try tmp.dir.readFileAlloc(io, "hello.txt", gpa, .limited(1024));
    defer gpa.free(content);

    try testing.expectEqualStrings("Hello, Zig!", content);
}

test "文件不存在时报错" {
    const io = testing.io;
    const gpa = testing.allocator;

    var tmp = testing.tmpDir(.{});
    defer tmp.cleanup();

    // 读取不存在的文件会返回 error.FileNotFound
    const result = tmp.dir.readFileAlloc(io, "not_exist.txt", gpa, .limited(1024));
    try testing.expectError(error.FileNotFound, result);
}

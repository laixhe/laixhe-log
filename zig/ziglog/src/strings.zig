const std = @import("std");
const expect = std.testing.expect;

// 字符串字面量：类型是 *const [N:0]u8，带 NUL 结尾
test "字符串字面量" {
    const s = "hello";
    try expect(s.len == 5);
    try expect(s[0] == 'h');
}

// 字符串比较：用 std.mem.eql
test "字符串比较" {
    const a = "hello";
    const b = "hello";
    try expect(std.mem.eql(u8, a, b));
}

// 格式化输出到缓冲区
test "格式化输出" {
    var buf: [64]u8 = undefined;
    const s = try std.fmt.bufPrint(&buf, "value = {d}", .{42});
    try expect(std.mem.eql(u8, s, "value = 42"));
}

// 拼接字符串（需要分配器，这里用固定缓冲区演示）
test "字符串拼接" {
    var buf: [64]u8 = undefined;
    const s = try std.fmt.bufPrint(&buf, "{s} {s}", .{ "hello", "world" });
    try expect(std.mem.eql(u8, s, "hello world"));
}

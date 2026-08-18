const std = @import("std");
const expect = std.testing.expect;

// 整数类型：iN 有符号，uN 无符号
test "整数类型" {
    const a: i32 = 42;
    const b: u8 = 255;
    const c: usize = 100; // 指针大小的无符号整数
    try expect(a == 42);
    try expect(b == 255);
    try expect(c == 100);
}

// 浮点与布尔
test "浮点与布尔" {
    const pi: f64 = 3.14159;
    const ok: bool = true;
    try expect(pi > 3.0);
    try expect(ok);
}

// 数组与切片
test "数组与切片" {
    const arr = [_]i32{ 1, 2, 3, 4, 5 }; // 数组字面量，长度自动推断
    const slice: []const i32 = &arr;       // 切片是对数组的视图
    try expect(slice.len == 5);
    try expect(slice[0] == 1);
    try expect(slice[4] == 5);
}

// 枚举
test "枚举" {
    const Color = enum { red, green, blue };
    const c = Color.green;
    try expect(c == Color.green);
}

// 结构体
test "结构体" {
    const Point = struct {
        x: i32,
        y: i32,
    };
    const p = Point{ .x = 3, .y = 4 };
    try expect(p.x == 3);
    try expect(p.y == 4);
}

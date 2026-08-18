const std = @import("std");
const testing = std.testing;

// comptime 是 Zig 的核心特性：在编译期求值，用于泛型、元编程和编译期计算。

// 1. 泛型函数：通过 comptime T: type 参数实现泛型
fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

test "泛型函数" {
    try testing.expectEqual(5, max(i32, 3, 5));
    try testing.expectEqual(3.5, max(f64, 3.5, 2.0));
}

// 2. 编译期计算：comptime 函数在编译期执行
fn fib(comptime n: usize) usize {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

test "编译期计算" {
    const result = comptime blk: {
        @setEvalBranchQuota(100_000); // 提高编译期分支配额（递归 fib 需要）
        break :blk fib(20);
    };
    try testing.expectEqual(6765, result);
}

// 3. 泛型结构体：comptime 函数返回 type
fn Pair(comptime T: type) type {
    return struct {
        first: T,
        second: T,
    };
}

test "泛型结构体" {
    const IntPair = Pair(i32);
    const p = IntPair{ .first = 1, .second = 2 };
    try testing.expectEqual(1, p.first);
    try testing.expectEqual(2, p.second);
}

// 4. 类型反射：@typeName 获取类型名
test "类型反射" {
    try testing.expectEqualStrings("i32", @typeName(i32));
    try testing.expectEqualStrings("u8", @typeName(u8));
}

// 5. 编译期类型检查：@typeInfo 判断类型，@compileError 抛出编译错误
fn onlyForIntegers(comptime T: type) void {
    switch (@typeInfo(T)) {
        .int, .comptime_int => {},
        else => @compileError("only integers are allowed"),
    }
}

test "编译期类型检查" {
    onlyForIntegers(i32); // 编译通过
    // onlyForIntegers(f32); // 取消注释会在编译期报错
}

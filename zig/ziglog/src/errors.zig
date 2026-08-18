const std = @import("std");
const expect = std.testing.expect;

// 返回错误联合（!i32 表示要么是 i32，要么是错误）
fn mayFail(flag: bool) !i32 {
    if (flag) {
        return error.SomeError;
    }
    return 42;
}

// error union 与 catch：捕获错误并处理
test "error union 与 catch" {
    const v = mayFail(false) catch |err| {
        std.debug.print("error: {s}\n", .{@errorName(err)});
        // mayFail(false) 预期不会出错，因此这里用 unreachable 表示「不会走到」
        unreachable;
    };
    try expect(v == 42);
}

// try 表达式：出错时直接向上传播
test "try 表达式" {
    const v = try mayFail(false);
    try expect(v == 42);
}

// 判断是否返回了特定错误
test "错误传播" {
    const result = mayFail(true);
    try expect(result == error.SomeError);
}

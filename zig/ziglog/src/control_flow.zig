const std = @import("std");
const expect = std.testing.expect;

// if 语句
test "if 语句" {
    const x = 10;
    var result: u8 = 0;
    if (x > 5) {
        result = 1;
    } else {
        result = 0;
    }
    try expect(result == 1);
}

// switch 语句：穷举且可返回值
test "switch 语句" {
    const x: u8 = 2;
    const name = switch (x) {
        1 => "one",
        2 => "two",
        else => "many",
    };
    try expect(std.mem.eql(u8, name, "two"));
}

// for 循环
test "for 循环" {
    const items = [_]i32{ 1, 2, 3 };
    var sum: i32 = 0;
    for (items) |item| {
        sum += item;
    }
    try expect(sum == 6);
}

// while 循环：使用 continue 表达式累加
test "while 循环" {
    var i: u8 = 0;
    var count: u8 = 0;
    while (i < 5) : (i += 1) {
        count += 1;
    }
    try expect(count == 5);
}

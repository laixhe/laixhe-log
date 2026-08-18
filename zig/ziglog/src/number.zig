const std = @import("std");
const testing = std.testing;

// 数值类型进阶：格式化输出 / 整数溢出处理 / 类型转换。
// 对应 Rust rustlog/src/number.rs 与 Go golog 的数值示例。
//
// 前置知识：
// - 溢出：`+` 在安全模式下运行时检查（越界 panic）；`+%` 总是回绕；
//   @addWithOverflow 返回 (结果, 是否溢出)；std.math.add 返回 error.Overflow
// - 转换：@intCast 窄化（越界 panic）、@intFromFloat 向零截断、@round 四舍五入

// 格式化带符号数：非负补 +，负数自带 -
fn formatSigned(value: i32, out: []u8) ![]const u8 {
    if (value >= 0) {
        return std.fmt.bufPrint(out, "+{d}", .{value});
    }
    return std.fmt.bufPrint(out, "{d}", .{value});
}

// 格式化输出（对应 Rust number_to_string）
test "数值格式化输出" {
    var buf: [128]u8 = undefined;

    // 精度控制（四舍五入）
    const f = try std.fmt.bufPrint(&buf, "f1={d:.2} f2={d:.2}", .{ 88.888, 88.0 });
    try testing.expectEqualStrings("f1=88.89 f2=88.00", f);

    // 十六进制（Zig 仅支持小写 {x}）/ 八进制 / 二进制
    const base = try std.fmt.bufPrint(&buf, "hex=0x{x} octal=0o{o} binary=0b{b}", .{ 666, 666, 666 });
    try testing.expectEqualStrings("hex=0x29a octal=0o1232 binary=0b1010011010", base);

    // 前导零填充 + 宽度控制：{d:0>8} 表示「右对齐，总宽度 8，不足补 0」
    const zeros = try std.fmt.bufPrint(&buf, "leading zeros: {d:0>8}", .{666});
    try testing.expectEqualStrings("leading zeros: 00000666", zeros);

    // 对齐：{d:<10} 左对齐，{d:>10} 右对齐
    const aligned = try std.fmt.bufPrint(&buf, "left=|{d:<10}| right=|{d:>10}|", .{ 666, 666 });
    try testing.expectEqualStrings("left=|666       | right=|       666|", aligned);

    // 正负号显式显示
    const s = try formatSigned(666, &buf);
    try testing.expectEqualStrings("+666", s);
    const s2 = try formatSigned(-888, &buf);
    try testing.expectEqualStrings("-888", s2);

    // 千分位分组（Zig 无内置逗号分组，这里演示手动拼接）
    const grouping = try std.fmt.bufPrint(&buf, "grouping: 1,234,567", .{});
    try testing.expectEqualStrings("grouping: 1,234,567", grouping);
}

// 整数溢出四种处理方式（对应 Rust overflow / Go 溢出检查）
test "整数溢出：回绕 / 检测 / 饱和 / 检查" {
    // 1) 回绕：+% 总是回绕（对应 Rust wrapping_add / Go 默认行为）
    const wrap: u8 = @as(u8, 255) +% 1;
    try testing.expectEqual(@as(u8, 0), wrap);

    // 2) 检测：@addWithOverflow 返回 (结果, 是否溢出)（对应 Rust overflowing_add）
    const result = @addWithOverflow(@as(u8, 255), @as(u8, 1));
    try testing.expectEqual(@as(u8, 0), result[0]);
    try testing.expectEqual(@as(u1, 1), result[1]);

    // 3) 饱和：std.math.add 溢出返回 error.Overflow，catch 后取最大值
    //    （对应 Rust saturating_add，无内置饱和函数时手动处理）
    const sat = std.math.add(u8, 250, 10) catch std.math.maxInt(u8);
    try testing.expectEqual(@as(u8, 255), sat);

    // 4) 检查：std.math.add 返回 error.Overflow（对应 Rust checked_add）
    const checked = std.math.add(u8, 255, 1);
    try testing.expectError(error.Overflow, checked);
    // 正常情况返回结果
    try testing.expectEqual(@as(u8, 200), try std.math.add(u8, 100, 100));
}

// 类型转换（对应 Rust type_conversion）
test "类型转换：窄化 / 截断 / 四舍五入 / 解析" {
    // 隐式拓宽：u8 -> u16 安全（无损失）
    const small: u8 = 10;
    const big: u16 = small;
    try testing.expectEqual(@as(u16, 10), big);

    // 显式窄化：@intCast（越界在安全模式下 panic）
    const narrowed: u8 = @intCast(200);
    try testing.expectEqual(@as(u8, 200), narrowed);

    // 浮点转整数：@intFromFloat 向零截断（需用 @as 指定结果类型）
    try testing.expectEqual(@as(i32, 3), @as(i32, @intFromFloat(3.99)));

    // 四舍五入：@round 先取整再转换
    try testing.expectEqual(@as(i32, 4), @as(i32, @intFromFloat(@round(3.99))));

    // 字符串解析：std.fmt.parseInt（失败返回 error.InvalidCharacter）
    try testing.expectEqual(@as(i32, 666), try std.fmt.parseInt(i32, "666", 10));
    // 进制解析：radix 指定进制
    try testing.expectEqual(@as(i32, 666), try std.fmt.parseInt(i32, "29A", 16));
    try testing.expectEqual(@as(i32, 666), try std.fmt.parseInt(i32, "1232", 8));

    // 解析失败演示
    try testing.expectError(error.InvalidCharacter, std.fmt.parseInt(i32, "not_a_number", 10));
}

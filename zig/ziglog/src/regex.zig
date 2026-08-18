const std = @import("std");
const testing = std.testing;
// Zig 标准库没有正则模块，这里引入第三方轻量库 mvzr（单文件、零依赖、MIT）。
// mvzr 的 Regex 支持 \d \w、字符集 [a-z]、量词 {m,n} + ? *、^ $ 等。
const mvzr = @import("lib/mvzr.zig");

// 说明：mvzr 不支持大小写不敏感修饰符（如 (?i)），
// 需要「大小写不敏感」时用 [A-Za-z] 字符类显式包含大小写。

test "正则匹配手机号码" {
    // 中国大陆手机号：1 开头，第二位 3-9，后面跟 9 位数字，共 11 位
    const phone_regex = mvzr.compile("^1[3-9]\\d{9}$").?;

    try testing.expect(phone_regex.isMatch("13812345678"));
    try testing.expect(phone_regex.isMatch("19912345678"));
    try testing.expect(!phone_regex.isMatch("12812345678")); // 第二位 2 非法
    try testing.expect(!phone_regex.isMatch("1381234567")); // 少一位（10 位）
    try testing.expect(!phone_regex.isMatch("138123456789")); // 多一位（12 位）
}

test "正则匹配邮箱（大小写不敏感）" {
    // 邮箱：本地部分 + @ + 域名 + .顶级域名
    // 大小写不敏感通过 [A-Za-z] 字符类实现
    const email_regex = mvzr.compile("^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$").?;

    try testing.expect(email_regex.isMatch("laixhe@example.com"));
    try testing.expect(email_regex.isMatch("LAIXHE@EXAMPLE.COM"));
    try testing.expect(email_regex.isMatch("LaixHe@Example.com")); // 混合大小写
    try testing.expect(email_regex.isMatch("user.name+tag@mail.example.org"));
    try testing.expect(!email_regex.isMatch("not-an-email")); // 无 @
    try testing.expect(!email_regex.isMatch("user@.com")); // 域名以点开头
    try testing.expect(!email_regex.isMatch("user@example")); // 缺少顶级域名
}

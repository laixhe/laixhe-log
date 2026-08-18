const std = @import("std");
const testing = std.testing;
const epoch = std.time.epoch;

// Zig 0.16 时间相关能力分两处：
// - std.Io.Timestamp：获取当前 Unix 时间戳（纳秒精度）
// - std.time.epoch：把时间戳（秒）换算成 年/月/日/时/分/秒
//
// 标准库只提供「时间戳 → 日历」的单向换算，
// 反过来「日历 → 时间戳」（即解析时间字符串）需要自己实现。

// 一个可读的日期时间对象
const DateTime = struct {
    year: epoch.Year, // u16，如 2026
    month: u4, // 1-12
    day: u5, // 1-31
    hour: u5, // 0-23
    minute: u6, // 0-59
    second: u6, // 0-59
};

// 时间戳（秒）→ 日期时间对象
fn fromEpochSeconds(secs: u64) DateTime {
    const es = epoch.EpochSeconds{ .secs = secs };
    const year_day = es.getEpochDay().calculateYearDay();
    const month_day = year_day.calculateMonthDay();
    const day_secs = es.getDaySeconds();
    return .{
        .year = year_day.year,
        .month = month_day.month.numeric(),
        .day = @intCast(month_day.day_index + 1), // day_index 从 0 开始，需 +1
        .hour = day_secs.getHoursIntoDay(),
        .minute = day_secs.getMinutesIntoHour(),
        .second = day_secs.getSecondsIntoMinute(),
    };
}

// 格式化为 "YYYY-MM-DD HH:MM:SS"
fn formatEpochSeconds(secs: u64, buf: []u8) ![]const u8 {
    const dt = fromEpochSeconds(secs);
    return std.fmt.bufPrint(buf, "{d:0>4}-{d:0>2}-{d:0>2} {d:0>2}:{d:0>2}:{d:0>2}", .{
        dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second,
    });
}

// 公历日期 → 自 1970-01-01 起的天数
// 采用 Howard Hinnant 的 days_from_civil 算法（无分支除法）
fn daysFromCivil(year: i64, month: i64, day: i64) i64 {
    const y = if (month <= 2) year - 1 else year;
    const era = @divFloor(y, 400);
    const yoe = y - era * 400; // 年份在纪元内的偏移 [0, 399]
    const mp = if (month > 2) month - 3 else month + 9; // 3 月作为一年起点 [0, 11]
    const doy = @divFloor((153 * mp + 2), 5) + day - 1; // 一年中的第几天 [0, 365]
    const doe = yoe * 365 + @divFloor(yoe, 4) - @divFloor(yoe, 100) + doy; // [0, 146096]
    return era * 146097 + doe - 719468;
}

// 日期时间 → 时间戳（秒）
fn dateTimeToEpochSeconds(year: i64, month: i64, day: i64, hour: i64, minute: i64, second: i64) i64 {
    const days = daysFromCivil(year, month, day);
    return days * 86400 + hour * 3600 + minute * 60 + second;
}

// 解析 "YYYY-MM-DD HH:MM:SS" 为时间戳（秒）
fn parseDateTime(s: []const u8) !u64 {
    if (s.len != 19) return error.InvalidFormat;
    // 校验分隔符位置
    if (s[4] != '-' or s[7] != '-' or s[10] != ' ' or s[13] != ':' or s[16] != ':')
        return error.InvalidFormat;

    const year = try std.fmt.parseInt(i64, s[0..4], 10);
    const month = try std.fmt.parseInt(i64, s[5..7], 10);
    const day = try std.fmt.parseInt(i64, s[8..10], 10);
    const hour = try std.fmt.parseInt(i64, s[11..13], 10);
    const minute = try std.fmt.parseInt(i64, s[14..16], 10);
    const second = try std.fmt.parseInt(i64, s[17..19], 10);

    const secs = dateTimeToEpochSeconds(year, month, day, hour, minute, second);
    if (secs < 0) return error.InvalidFormat;
    return @intCast(secs);
}

test "获取当前时间戳（秒级）" {
    const io = testing.io;
    const secs = std.Io.Timestamp.now(io, .real).toSeconds();
    try testing.expect(secs > 0);
}

test "当前时间格式化为 YYYY-MM-DD HH:MM:SS" {
    const io = testing.io;
    const secs: u64 = @intCast(std.Io.Timestamp.now(io, .real).toSeconds());
    var buf: [32]u8 = undefined;
    const s = try formatEpochSeconds(secs, &buf);

    // 校验长度与分隔符
    try testing.expectEqual(@as(usize, 19), s.len);
    try testing.expectEqual('-', s[4]);
    try testing.expectEqual('-', s[7]);
    try testing.expectEqual(' ', s[10]);
    try testing.expectEqual(':', s[13]);
    try testing.expectEqual(':', s[16]);
}

test "时间戳转时间对象" {
    // 1625159473 对应 2021-07-01 17:11:13
    const dt = fromEpochSeconds(1625159473);
    try testing.expectEqual(@as(u16, 2021), dt.year);
    try testing.expectEqual(@as(u4, 7), dt.month);
    try testing.expectEqual(@as(u5, 1), dt.day);
    try testing.expectEqual(@as(u5, 17), dt.hour);
    try testing.expectEqual(@as(u6, 11), dt.minute);
    try testing.expectEqual(@as(u6, 13), dt.second);
}

test "格式化固定时间戳" {
    var buf: [32]u8 = undefined;
    const s = try formatEpochSeconds(1625159473, &buf);
    try testing.expectEqualStrings("2021-07-01 17:11:13", s);
}

test "时间字符串解析为时间戳" {
    const secs = try parseDateTime("2021-07-01 17:11:13");
    try testing.expectEqual(@as(u64, 1625159473), secs);
}

test "时间比较" {
    const t1 = fromEpochSeconds(1625159473); // 2021-07-01 17:11:13
    const t2 = fromEpochSeconds(1625159474); // 晚 1 秒

    // 最直观：比较时间戳数值即可判断先后
    try testing.expect(1625159473 < 1625159474);

    // 或比较拆解后的字段（先年、再月/日/时/分/秒）
    try testing.expectEqual(t1.year, t2.year);
    try testing.expectEqual(t1.hour, t2.hour);
    try testing.expect(t1.second < t2.second);
}

test "测量耗时（单调时钟）" {
    const io = testing.io;
    // 单调时钟不受系统时间调整影响，适合测量时长
    const start = std.Io.Timestamp.now(io, .awake);
    const end = std.Io.Timestamp.now(io, .awake);
    const duration = start.durationTo(end);
    try testing.expect(duration.toNanoseconds() >= 0);
}

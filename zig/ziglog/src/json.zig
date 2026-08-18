const std = @import("std");
const testing = std.testing;

// Zig 0.16 的 JSON 处理：
// - 序列化：std.json.Stringify.valueAlloc(gpa, value, options) 返回 JSON 字符串
// - 反序列化：std.json.parseFromSlice(T, gpa, json, options) 返回 Parsed(T)

const User = struct {
    id: u32,
    name: []const u8,
};

test "序列化结构体为 JSON" {
    const user = User{ .id = 1, .name = "laixhe" };

    // valueAlloc 返回分配好的 JSON 字符串，需手动释放
    const json_str = try std.json.Stringify.valueAlloc(testing.allocator, user, .{});
    defer testing.allocator.free(json_str);

    try testing.expectEqualStrings("{\"id\":1,\"name\":\"laixhe\"}", json_str);
}

test "反序列化 JSON 为结构体" {
    const json_str = "{\"id\":2,\"name\":\"zig\"}";

    // parseFromSlice 返回 Parsed(T)，需 defer deinit
    const parsed = try std.json.parseFromSlice(User, testing.allocator, json_str, .{});
    defer parsed.deinit();

    try testing.expectEqual(2, parsed.value.id);
    try testing.expectEqualStrings("zig", parsed.value.name);
}

test "序列化嵌套结构体" {
    const Group = struct {
        name: []const u8,
        users: []const User,
    };

    const group = Group{
        .name = "team",
        .users = &[_]User{
            .{ .id = 1, .name = "a" },
            .{ .id = 2, .name = "b" },
        },
    };

    const json_str = try std.json.Stringify.valueAlloc(testing.allocator, group, .{});
    defer testing.allocator.free(json_str);

    try testing.expectEqualStrings(
        "{\"name\":\"team\",\"users\":[{\"id\":1,\"name\":\"a\"},{\"id\":2,\"name\":\"b\"}]}",
        json_str,
    );
}

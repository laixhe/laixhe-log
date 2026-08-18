const std = @import("std");
const testing = std.testing;

// Zig 的内存管理是显式的：通过 Allocator 接口分配和释放，没有垃圾回收。
// 测试环境用 std.testing.allocator（带泄漏检测）。

test "分配和释放数组" {
    const allocator = testing.allocator;

    // 分配 10 个 i32 的切片
    const arr = try allocator.alloc(i32, 10);
    defer allocator.free(arr); // defer 确保函数返回前释放

    arr[0] = 42;
    arr[9] = 99;
    try testing.expect(arr[0] == 42);
    try testing.expect(arr[9] == 99);
    try testing.expect(arr.len == 10);
}

test "创建和销毁单个对象" {
    const allocator = testing.allocator;

    const obj = try allocator.create(i32);
    defer allocator.destroy(obj);

    obj.* = 123;
    try testing.expect(obj.* == 123);
}

test "ArenaAllocator 批量分配" {
    // ArenaAllocator：所有分配集中管理，deinit 时一次性释放全部
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();

    const a = arena.allocator();

    const s1 = try a.alloc(u8, 100);
    const s2 = try a.alloc(u8, 200);
    try testing.expect(s1.len == 100);
    try testing.expect(s2.len == 200);
    // 无需逐个 free，arena.deinit() 会统一释放
}

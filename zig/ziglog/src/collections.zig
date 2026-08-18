const std = @import("std");
const testing = std.testing;
const allocator = testing.allocator;

// 容器进阶：动态数组 / 字典 / 去重 / 双向链表 / 堆 / 环形缓冲 / LRU。
// 对应 Go golog 的 container/list、container/ring、container/heap、lru_test.go、
// slice_test.go、map_test.go、unique_test.go 与 Rust array_map.rs 的进阶部分。
//
// 前置知识：
// - std.ArrayList(T) 对应 Go slice / Rust Vec（动态数组）
// - std.StringHashMap(V) 对应 Go map / Rust HashMap
// - std.DoublyLinkedList 对应 Go container/list（0.16 起替代 std.LinkedList）
// - std.PriorityQueue 对应 Go container/heap（0.16 起比较函数返回 std.math.Order）

// ---------- 动态数组 ArrayList（对应 Go slice_test.go / Rust Vec）----------
test "动态数组：查找 / 排序 / 反转" {
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(allocator);

    try list.append(allocator, 3);
    try list.append(allocator, 1);
    try list.append(allocator, 2);

    // 查找（对应 slices.Contains / indexOf）
    const has2 = std.mem.indexOfScalar(i32, list.items, 2) != null;
    try testing.expect(has2);

    // 排序（对应 slices.Sort，原地升序）
    std.sort.block(i32, list.items, {}, std.sort.asc(i32));
    try testing.expectEqualSlices(i32, &.{ 1, 2, 3 }, list.items);

    // 反转（对应 slices.Reverse，从大到小）
    std.mem.reverse(i32, list.items);
    try testing.expectEqualSlices(i32, &.{ 3, 2, 1 }, list.items);

    // 移除连续重复（对应 slices.Compact，只去除相邻重复）
    var compacted: std.ArrayList(i32) = .empty;
    defer compacted.deinit(allocator);
    const compact = [_]i32{ 11, 2, 2, 3, 3, 8, 11 };
    for (compact, 0..) |n, i| {
        if (i == 0 or n != compact[i - 1]) {
            try compacted.append(allocator, n);
        }
    }
    try testing.expectEqualSlices(i32, &.{ 11, 2, 3, 8, 11 }, compacted.items);
}

// 结构体排序（对应 slices.SortFunc：年龄升序、相同按姓名升序）
const Person = struct {
    name: []const u8,
    age: u8,
};

fn personLess(_: void, a: Person, b: Person) bool {
    if (a.age != b.age) return a.age < b.age;
    return std.mem.lessThan(u8, a.name, b.name);
}

test "结构体排序（年龄升序、姓名次之）" {
    var people = [_]Person{
        .{ .name = "Jax", .age = 36 },
        .{ .name = "TJ", .age = 26 },
        .{ .name = "Alex", .age = 76 },
    };
    std.sort.block(Person, &people, {}, personLess);
    try testing.expectEqualStrings("TJ", people[0].name);
    try testing.expectEqualStrings("Jax", people[1].name);
    try testing.expectEqualStrings("Alex", people[2].name);
}

// ---------- 字典 StringHashMap（对应 Go map_test.go / Rust HashMap）----------
test "字典：增删查改与有序 key" {
    var map = std.StringHashMap(i32).init(allocator);
    defer map.deinit();

    try map.put("a", 1);
    try map.put("b", 2);
    try map.put("c", 3);

    // 查询（对应 m["b"] / get）
    try testing.expectEqual(@as(i32, 2), map.get("b").?);
    try testing.expect(map.contains("a"));

    // 删除（对应 delete）
    _ = map.remove("c");
    try testing.expect(!map.contains("c"));

    // 不存在则插入默认值（对应 entry().or_insert）
    const entry = try map.getOrPut("b");
    if (!entry.found_existing) entry.value_ptr.* = 20;
    try testing.expectEqual(@as(i32, 2), map.get("b").?); // 已存在，保持 2

    // 排序后的 key（对应 slices.Sorted(maps.Keys(m))）
    var keys: std.ArrayList([]const u8) = .empty;
    defer keys.deinit(allocator);
    var it = map.keyIterator();
    while (it.next()) |k| try keys.append(allocator, k.*);
    std.sort.block([]const u8, keys.items, {}, stringLessThan);

    try testing.expectEqualStrings("a", keys.items[0]);
    try testing.expectEqualStrings("b", keys.items[1]);
}

fn stringLessThan(_: void, a: []const u8, b: []const u8) bool {
    return std.mem.lessThan(u8, a, b);
}

// ---------- 去重（对应 Go unique_test.go）----------
test "去重：AutoHashMap 记录已出现元素" {
    const nums = [_]i32{ 3, 1, 2, 1, 3, 2, 4, 5, 4 };

    var seen = std.AutoHashMap(i32, void).init(allocator);
    defer seen.deinit();

    var result: std.ArrayList(i32) = .empty;
    defer result.deinit(allocator);

    for (nums) |n| {
        const gop = try seen.getOrPut(n);
        if (!gop.found_existing) {
            try result.append(allocator, n); // 首次出现才保留（保持顺序）
        }
    }
    try testing.expectEqualSlices(i32, &.{ 3, 1, 2, 4, 5 }, result.items);
}

// ---------- 双向链表 DoublyLinkedList（对应 Go container/list）----------
test "双向链表：队首 / 队尾 / 弹出" {
    const Node = struct {
        data: i32,
        node: std.DoublyLinkedList.Node = .{},
    };

    var list: std.DoublyLinkedList = .{};
    var n1 = Node{ .data = 1 };
    var n2 = Node{ .data = 2 };
    var n3 = Node{ .data = 3 };

    list.append(&n1.node); // 尾部插入（PushBack）
    list.prepend(&n2.node); // 头部插入（PushFront）
    list.append(&n3.node);

    try testing.expectEqual(@as(usize, 3), list.len());

    // 头部弹出（对应 queue.Front() + Remove）
    const front = list.popFirst().?;
    const front_node: *Node = @fieldParentPtr("node", front);
    try testing.expectEqual(@as(i32, 2), front_node.data);
    try testing.expectEqual(@as(usize, 2), list.len());
}

// ---------- 堆 PriorityQueue（对应 Go container/heap 默认最小堆）----------
fn minOrder(_: void, a: i32, b: i32) std.math.Order {
    return std.math.order(a, b); // 越小优先级越高 = 最小堆
}

fn maxOrder(_: void, a: i32, b: i32) std.math.Order {
    return std.math.order(b, a); // 越大优先级越高 = 最大堆
}

test "堆：最小堆与最大堆" {
    // 最小堆（默认，对应 Go container/heap）
    var min_heap = std.PriorityQueue(i32, void, minOrder).initContext({});
    defer min_heap.deinit(allocator);

    try min_heap.push(allocator, 30);
    try min_heap.push(allocator, 10);
    try min_heap.push(allocator, 50);
    try min_heap.push(allocator, 20);

    try testing.expectEqual(@as(i32, 10), min_heap.peek().?); // 堆顶(最小)
    try testing.expectEqual(@as(i32, 10), min_heap.pop().?);
    try testing.expectEqual(@as(i32, 20), min_heap.pop().?);

    // 最大堆（反转比较函数）
    var max_heap = std.PriorityQueue(i32, void, maxOrder).initContext({});
    defer max_heap.deinit(allocator);

    try max_heap.push(allocator, 30);
    try max_heap.push(allocator, 10);
    try max_heap.push(allocator, 50);
    try max_heap.push(allocator, 20);

    try testing.expectEqual(@as(i32, 50), max_heap.pop().?);
    try testing.expectEqual(@as(i32, 30), max_heap.pop().?);
}

// ---------- 环形缓冲区 RingBuffer（对应 Go container/ring）----------
const RingBuffer = struct {
    buf: []i32,
    capacity: usize,
    head: usize = 0, // 下一个写入位置
    size: usize = 0,

    fn init(alloc: std.mem.Allocator, capacity: usize) !RingBuffer {
        return .{ .buf = try alloc.alloc(i32, capacity), .capacity = capacity };
    }

    fn deinit(self: *RingBuffer, alloc: std.mem.Allocator) void {
        alloc.free(self.buf);
    }

    // 写入：环满则覆盖最旧数据
    fn add(self: *RingBuffer, value: i32) void {
        self.buf[self.head] = value;
        self.head = (self.head + 1) % self.capacity;
        if (self.size < self.capacity) self.size += 1;
    }

    // 从最旧到最新遍历
    fn snapshot(self: *const RingBuffer, out: []i32) void {
        // head + capacity - size 恒非负（避免 usize 下溢）
        const start = (self.head + self.capacity - self.size) % self.capacity;
        for (0..self.size) |i| {
            out[i] = self.buf[(start + i) % self.capacity];
        }
    }
};

test "环形缓冲区：写满覆盖最旧数据" {
    var ring = try RingBuffer.init(allocator, 5);
    defer ring.deinit(allocator);

    for (0..5) |i| ring.add(@intCast(i)); // 写入 0-4
    var snap: [5]i32 = undefined;
    ring.snapshot(&snap);
    try testing.expectEqualSlices(i32, &.{ 0, 1, 2, 3, 4 }, &snap);

    // 继续写入 3 个，最旧的 0、1、2 被覆盖
    ring.add(5);
    ring.add(6);
    ring.add(7);
    ring.snapshot(&snap);
    try testing.expectEqualSlices(i32, &.{ 3, 4, 5, 6, 7 }, &snap);
}

// ---------- LRU 缓存（对应 Go lru_test.go：双向链表 + 字典）----------
const LRUCache = struct {
    const Node = struct {
        key: []const u8,
        value: []const u8,
        node: std.DoublyLinkedList.Node = .{},
    };

    allocator: std.mem.Allocator,
    capacity: usize,
    cache: std.StringHashMap(*Node), // key -> 节点（O(1) 查找）
    list: std.DoublyLinkedList = .{}, // 队首 = 最久未使用，队尾 = 最近使用

    fn init(alloc: std.mem.Allocator, capacity: usize) LRUCache {
        return .{
            .allocator = alloc,
            .capacity = capacity,
            .cache = std.StringHashMap(*Node).init(alloc),
        };
    }

    fn deinit(self: *LRUCache) void {
        var it = self.cache.valueIterator();
        while (it.next()) |node| {
            self.allocator.free(node.*.key);
            self.allocator.free(node.*.value);
            self.allocator.destroy(node.*);
        }
        self.cache.deinit();
    }

    fn get(self: *LRUCache, key: []const u8) ?[]const u8 {
        const node = self.cache.get(key) orelse return null;
        // 访问后移到队尾，提升优先级（对应 MoveToBack）
        self.list.remove(&node.node);
        self.list.append(&node.node);
        return node.value;
    }

    fn put(self: *LRUCache, key: []const u8, value: []const u8) !void {
        if (self.cache.get(key)) |node| {
            const old_value = node.value;
            node.value = try self.allocator.dupe(u8, value);
            self.allocator.free(old_value);
            self.list.remove(&node.node);
            self.list.append(&node.node);
            return;
        }

        // 超出容量时淘汰队首（最久未使用）
        if (self.cache.count() >= self.capacity) {
            const oldest = self.list.popFirst().?;
            const node: *Node = @fieldParentPtr("node", oldest);
            _ = self.cache.remove(node.key);
            self.allocator.free(node.key);
            self.allocator.free(node.value);
            self.allocator.destroy(node);
        }

        const node = try self.allocator.create(Node);
        node.* = .{
            .key = try self.allocator.dupe(u8, key),
            .value = try self.allocator.dupe(u8, value),
        };
        try self.cache.put(node.key, node);
        self.list.append(&node.node);
    }

    // 从最久未使用到最近使用遍历
    fn entries(self: *LRUCache, out: *std.ArrayList([2][]const u8)) !void {
        out.clearRetainingCapacity();
        var it = self.list.first;
        while (it) |n| {
            const node: *Node = @fieldParentPtr("node", n);
            try out.append(self.allocator, .{ node.key, node.value });
            it = n.next;
        }
    }
};

test "LRU 缓存：容量淘汰最久未使用" {
    var cache = LRUCache.init(allocator, 3);
    defer cache.deinit();

    try cache.put("key1", "value1");
    try cache.put("key2", "value2");
    try cache.put("key3", "value3");

    // 访问 key2，使其变为最近使用
    try testing.expectEqualStrings("value2", cache.get("key2").?);

    // 容量 3，插入 key4 应淘汰 key1（最久未使用）
    try cache.put("key4", "value4");
    try testing.expectEqual(@as(?[]const u8, null), cache.get("key1"));
    try testing.expectEqualStrings("value2", cache.get("key2").?); // 再次访问 key2 → 移到队尾

    // 遍历顺序：key3 → key4 → key2（key1 已被淘汰；key2 最近再次被访问）
    var entries: std.ArrayList([2][]const u8) = .empty;
    defer entries.deinit(allocator);
    try cache.entries(&entries);
    try testing.expectEqual(@as(usize, 3), entries.items.len);
    try testing.expectEqualStrings("key3", entries.items[0][0]);
    try testing.expectEqualStrings("key4", entries.items[1][0]);
    try testing.expectEqualStrings("key2", entries.items[2][0]);
}

const std = @import("std");
const testing = std.testing;

// 并发同步：线程（std.Thread）/ 互斥锁（std.Io.Mutex）/ 原子操作（@atomicRmw）。
// 对应 Go golog/sync_test.go。
//
// 对应关系：
// - sync.Once      → 双检锁模式（Lock + bool 标志）
// - sync.WaitGroup → std.Thread.spawn + join() 等待所有线程结束
// - sync.Mutex     → std.Io.Mutex（lock / unlock，需要 io 参数）
// - sync.RWMutex   → 读多写少场景用普通 Mutex 即可（Zig 无内置 RWMutex）
// - sync/atomic    → @atomicRmw（无锁原子自增）

// 一个带锁的共享计数器（对应 Go 的 `var mu sync.Mutex; var count int`）
const Counter = struct {
    mutex: std.Io.Mutex = std.Io.Mutex.init,
    value: u32 = 0,

    fn incr(self: *Counter, io: std.Io) void {
        // 加锁（对应 mu.Lock()），失败则忽略（Cancelable 语义）
        self.mutex.lock(io) catch return;
        defer self.mutex.unlock(io); // 解锁（对应 mu.Unlock()）
        self.value += 1; // 临界区：同一时刻只有一个线程能执行
    }
};

// 只执行一次（对应 sync.Once）
const OnceState = struct {
    mutex: std.Io.Mutex = std.Io.Mutex.init,
    done: bool = false,
    count: u32 = 0,

    fn doOnce(self: *OnceState, io: std.Io) void {
        if (self.done) return; // 第一次检查（快路径）
        self.mutex.lock(io) catch return;
        defer self.mutex.unlock(io);
        if (!self.done) { // 第二次检查（加锁后）
            self.done = true;
            self.count += 1;
        }
    }
};

// 每个线程执行 n 次自增（用于 WaitGroup / Mutex / atomic 测试）
fn incrNTimes(counter: *Counter, io: std.Io, n: u32) void {
    for (0..n) |_| counter.incr(io);
}

fn atomicIncrNTimes(ptr: *u32, n: u32) void {
    for (0..n) |_| {
        // 原子自增（对应 count.Add(1)），无需加锁
        _ = @atomicRmw(u32, ptr, .Add, 1, .monotonic);
    }
}

test "只执行一次（对应 sync.Once）" {
    const io = testing.io;
    var state = OnceState{};

    // 5 个线程同时调用 doOnce，但初始化只会执行一次
    const Worker = struct {
        fn run(s: *OnceState, io2: std.Io) void {
            s.doOnce(io2);
        }
    };

    var threads: [5]std.Thread = undefined;
    for (&threads) |*t| {
        t.* = try std.Thread.spawn(.{}, Worker.run, .{ &state, io });
    }
    for (&threads) |*t| t.join();

    try testing.expectEqual(@as(u32, 1), state.count); // 结果: count = 1
}

test "等待一组线程完成（对应 sync.WaitGroup）" {
    const io = testing.io;
    var counter = Counter{};

    // 5 个线程各自完成一次任务，主线程 join 等待全部结束
    var threads: [5]std.Thread = undefined;
    for (&threads) |*t| {
        t.* = try std.Thread.spawn(.{}, incrNTimes, .{ &counter, io, @as(u32, 1) });
    }
    for (&threads) |*t| t.join(); // 对应 wg.Wait()：阻塞直到所有线程完成

    try testing.expectEqual(@as(u32, 5), counter.value); // 所有线程完成
}

test "互斥锁保护共享变量（对应 sync.Mutex）" {
    const io = testing.io;
    var counter = Counter{};

    // 10 个线程 × 100 次自增，加锁保证结果正确
    var threads: [10]std.Thread = undefined;
    for (&threads) |*t| {
        t.* = try std.Thread.spawn(.{}, incrNTimes, .{ &counter, io, @as(u32, 100) });
    }
    for (&threads) |*t| t.join();

    try testing.expectEqual(@as(u32, 1000), counter.value); // 结果: count = 1000（不加锁可能错误）
}

test "原子操作（对应 sync/atomic）" {
    var count: u32 = 0;

    // 10 个线程 × 100 次原子自增，无需加锁
    var threads: [10]std.Thread = undefined;
    for (&threads) |*t| {
        t.* = try std.Thread.spawn(.{}, atomicIncrNTimes, .{ &count, @as(u32, 100) });
    }
    for (&threads) |*t| t.join();

    try testing.expectEqual(@as(u32, 1000), count); // 结果: count = 1000
}

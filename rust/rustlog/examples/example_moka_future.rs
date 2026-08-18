//! 异步内存缓存示例（moka::future::Cache）—— 在 tokio 异步运行时下使用。
//!
//! ## 前置知识
//! moka 分 4 种 API，对应用户场景：
//! | 类型 | 同步/异步 | 用途 |
//! |---|---|---|
//! | `sync::Cache`   | 同步 | 普通多线程代码（本项目的 example_moka_sync） |
//! | `future::Cache` | 异步 | tokio / async-std 异步代码（本文件） |
//! | `sync::SegmentedCache` | 同步 | 极高并发（写入特别多），减少锁竞争 |
//! | `future::SegmentedCache` | 异步 | 极高并发异步场景 |
//!
//! ## 练习题
//! 1. 开 100 个 tokio 异步任务并发 `get_with` 同一个 key，观察函数体是否只执行 1 次（原子填充防击穿）。
//! 2. 用 `moka::notification` 的 `InvalidationListener`，打印每个被淘汰条目的淘汰原因。
//! 3. 对比 `get_with`（异步闭包）和 `try_get_with`（返回 Result 的异步闭包）的区别。

use moka::future::Cache;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 构建异步缓存：最大容量 1000，TTL 60 秒
    let cache: Cache<&str, &str> = Cache::builder()
        .max_capacity(1000)
        .time_to_live(Duration::from_secs(60))
        .build();

    // 异步插入
    cache.insert("user:100", "赵六").await;

    // 异步获取
    if let Some(name) = cache.get(&"user:100").await {
        println!("异步 get user:100 → {}", name);
    }

    // 异步删除
    cache.invalidate(&"user:100").await;
    println!("invalidate user:100 后查询 → {:?}", cache.get(&"user:100").await);

    // ⭐ get_with：原子填充（避免缓存击穿，最常用的 API 之一）
    // 场景：缓存 miss 时，从数据库 / 远程服务加载数据并插入。
    //
    // 为什么不自己「get 空 → 查库 → insert」？
    // 因为高并发下多个任务同时 miss，就会同时查库（缓存击穿）。
    // get_with 内部做了同步：同一时刻同一个 key 的闭包只会执行 1 次，其他任务等待结果。
    println!("\n==== get_with 原子填充 ====");
    let user = cache
        .get_with("user:101", async {
            println!("  [模拟 DB 查询 user:101 ... 耗时 50ms]");
            tokio::time::sleep(Duration::from_millis(50)).await;
            "孙七" // 返回值会被自动插入缓存
        })
        .await;
    println!("原子填充 user:101 → {}", user);

    // 第二次 get：直接命中缓存，不会再执行上面的 DB 查询
    let again = cache.get(&"user:101").await.expect("刚刚插入过，必然命中");
    println!("二次查询 user:101 → {}（未打印 DB 查询提示，证明走缓存）", again);

    println!("\n✅ 异步缓存示例完成");
}

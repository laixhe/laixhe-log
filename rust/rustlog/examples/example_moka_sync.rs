//! 同步内存缓存示例（moka::sync::Cache）。
//!
//! ## 前置知识
//! moka 是 Rust 生态高性能内存缓存，内部实现：
//! - 分段的 LRU + TinyLFU 准入策略（命中率高）
//! - **线程安全**：Cache 实例可直接 clone（内部是 Arc），**不需要 Mutex 包裹**
//! - get 会返回 value 的克隆副本，若 value 很大建议用 `Arc<T>` 包一层减克隆
//!
//! 过期策略两种（可同时启用）：
//! - **TTL（Time-To-Live）**：从插入时刻算起的绝对存活时间
//! - **TTI（Time-To-Idle）**：从「最后一次访问」算起的空闲时间
//!
//! 更多生产注意事项见项目根目录的 [moka.md](../../moka.md)。
//!
//! ## 练习题
//! 1. 把 value 换成一个 10MB 的大字符串，用 `Arc<String>` 包一层后再测。
//! 2. 设置 `per_key_weight` 权重，让大 key 占多份容量名额。
//! 3. 查文档 `eviction_listener`：条目被淘汰时触发回调，可用于日志统计。

use moka::sync::Cache;
use std::time::Duration;

fn main() {
    println!("============= sync_limit（容量限制） =============");
    sync_limit();
    println!("\n============= sync_time_limit（TTL + TTI） =============");
    sync_time_limit();
    println!("\n============= sync_stats（统计信息 + invalidate_all） =============");
    sync_stats();
}

fn sync_limit() {
    // 构建缓存：最大容量 1000，超过按 TinyLFU 算法淘汰
    let cache: Cache<&str, &str> = Cache::builder()
        .max_capacity(1000) // 最大条目数（key 数）
        .build();

    // 插入
    cache.insert("user:1", "张三");
    cache.insert("user:2", "李四");
    cache.insert("user:3", "王五");

    // 查询（返回 Option<T>；内部会克隆 value）
    if let Some(name) = cache.get(&"user:1") {
        println!("获取缓存 user:1 → {}", name);
    }

    // 删除数据：两种风格
    cache.invalidate(&"user:2");         // invalidate：无返回值
    let removed = cache.remove("user:1"); // remove：返回被删值的克隆
    println!("remove user:1 → 被删除的值 = {:?}", removed);

    // 看当前条目数（max_capacity 在 moka 0.12 中作为构建参数传入，
    // 没有 getter 方法；构建时设定值 = 1000，演示 entry_count() 即可）
    println!(
        "当前 entry_count = {}  /  构建时 max_capacity 设置为 1000",
        cache.entry_count(),
    );

    // invalidate_all：清空所有（谨慎！生产环境误操作容易引发缓存雪崩）
    cache.invalidate_all();
    println!("invalidate_all 后 entry_count = {}", cache.entry_count());
}

fn sync_time_limit() {
    // 同时启用 TTL（60s 绝对过期） + TTI（30s 空闲过期）
    // 任一条件先到，条目即失效
    let cache: Cache<&str, &str> = Cache::builder()
        .max_capacity(100)
        .time_to_live(Duration::from_secs(60))  // 插入后，无论访问与否 60 秒到期
        .time_to_idle(Duration::from_secs(30))  // 最后一次访问后，30 秒内再无访问则失效
        .build();

    cache.insert("key1", "value1");

    // 5 秒后访问：刷新 TTI 计时器（但不影响 TTL）
    std::thread::sleep(Duration::from_secs(5));
    if let Some(val) = cache.get(&"key1") {
        println!("5s 后访问 key1 → {}（TTI 从 0 重新计时）", val);
    }

    // 再等 35 秒：距离「上次访问」35 秒 > TTI(30s)，所以被 TTI 淘汰
    // 注意：这时 TTL 还剩 60 - (5+35) = 20 秒，但 TTI 先到了
    std::thread::sleep(Duration::from_secs(35));
    if cache.get(&"key1").is_none() {
        println!("40s 后 key1 已失效（TTL 未到，但 TTI 先超时）");
    }

    // 单独测试 TTL：不访问它，等 65 秒（超过 TTL 60s）
    cache.insert("key2", "value2");
    std::thread::sleep(Duration::from_secs(2)); // 为了让示例快点跑完，这里不等 65s；生产中可自行验证
    println!("(演示说明：若 sleep 65s，key2 会因 TTL 超时而失效)");
    drop(cache); // 提前释放（仅示意）
}

fn sync_stats() {
    // ---- 知识点：moka 0.12 默认没有 hit_count/miss_count/hit_rate 公共 API ----
    // 若要统计命中率，通常做法：业务层自己用 AtomicUsize 计数（或接入 metrics 库如 metrics/prometheus）
    //
    // ⚠️ 新手提示：下面这段代码看起来较长，核心只需记住一句话——
    //    「moka 0.12 没有内置命中率统计，需要自己在 get 外面包一层计数器」。
    //    如果暂时看不懂 AtomicUsize / 生命周期标注，跳过本函数完全没问题，不影响后续学习。
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CacheWithStats<'a> {
        cache: &'a Cache<i32, &'static str>,
        hits:  AtomicUsize,
        miss:  AtomicUsize,
    }
    impl<'a> CacheWithStats<'a> {
        fn get(&self, key: &i32) -> Option<&'static str> {
            match self.cache.get(key) {
                Some(v) => { self.hits.fetch_add(1, Ordering::Relaxed); Some(v) }
                None    => { self.miss.fetch_add(1, Ordering::Relaxed); None }
            }
        }
        fn report(&self) {
            let h = self.hits.load(Ordering::Relaxed);
            let m = self.miss.load(Ordering::Relaxed);
            let total = h + m;
            let rate = if total == 0 { 0.0 } else { h as f64 / total as f64 * 100.0 };
            println!("hit  = {h}");       // 3
            println!("miss = {m}");       // 2
            println!("命中率 ≈ {rate:.1}%"); // ≈ 60.0%
        }
    }

    let inner: Cache<i32, &str> = Cache::builder().max_capacity(100).build();
    inner.insert(1, "A");
    inner.insert(2, "B");
    let cache = CacheWithStats { cache: &inner, hits: AtomicUsize::new(0), miss: AtomicUsize::new(0) };

    // 模拟 3 次命中 + 2 次未命中
    cache.get(&1); cache.get(&1); cache.get(&2);  // hit * 3
    cache.get(&99); cache.get(&100);              // miss * 2
    cache.report();
}

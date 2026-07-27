use moka::sync::Cache;
use std::time::Duration;

fn main(){
    sync_limit();
    println!("============= sync_limit =============");
    sync_time_limit();
}

fn sync_limit() {
    // 创建缓存：最大容量1000条数据，超过容量将按 TinyLFU 算法淘汰
    let cache = Cache::builder()
        .max_capacity(1000) // 核心配置：缓存最大条目数
        .build();

    // 插入数据
    cache.insert("user:1", "张三");
    cache.insert("user:2", "李四");

    // 获取数据（返回 Option<T>，不存在则返回 None）
    if let Some(name) = cache.get(&"user:1") {
        println!("获取缓存：{}", name);
    }

    // 删除指定数据（两种方式：invalidate 无返回值，remove 会返回被删除的值）
    // invalidate 删除数据无返回值
    cache.invalidate(&"user:2");
    // remove 删除数据并返回被删除的值
    let removed = cache.remove("user:1"); // 返回 Option<T>，此处为 Some("王五")
    println!("被删除的值：{:?}", removed);

    // 清空所有缓存，谨慎使用，生产环境需避免误操作
    cache.invalidate_all();

    //  获取缓存当前条目数
    println!("缓存当前大小：{}", cache.entry_count());
}

// 带过期时间的缓存
// TTL（Time-To-Live）：从数据插入时开始计算，固定存活时间，到期后自动淘汰
// TTI（Time-To-Idle）：从数据最后一次访问时开始计算，空闲时间超过设定值则淘汰，适合“长期不访问即失效”的场景
fn sync_time_limit() {
    // 创建带过期策略的缓存：存活60秒（TTL），空闲30秒（TTI）
    let cache = Cache::builder()
        .max_capacity(100)
        .time_to_live(Duration::from_secs(60)) // 插入后，60秒内有效
        .time_to_idle(Duration::from_secs(30)) // 最后一次访问后，30秒内无访问则失效
        .build();

    cache.insert("key1", "value1");

    // 5秒后访问，刷新 TTI 计时
    std::thread::sleep(Duration::from_secs(5));
    if let Some(val) = cache.get(&"key1") {
        println!("5秒后访问：{}", val); // 输出：5秒后访问：value1
    }

    // 再等待35秒（累计空闲35秒，超过 TTI 30秒）
    std::thread::sleep(Duration::from_secs(35));
    if cache.get(&"key1").is_none() {
        println!("35秒后，缓存已失效（TTL未到，但TTI超时）");
    }

    // 插入新数据，测试 TTL
    cache.insert("key2", "value2");
    std::thread::sleep(Duration::from_secs(65)); // 超过 TTL 60秒
    if cache.get(&"key2").is_none() {
        println!("65秒后，缓存已失效（TTL超时）");
    }
}

use moka::future::Cache;
use std::time::Duration;

#[tokio::main]
async fn main() {
    //创建异步缓存：最大容量1000，TTL 60秒
    let cache = Cache::builder()
        .max_capacity(1000)
        .time_to_live(Duration::from_secs(60))
        .build();

    // 异步插入
    cache.insert("user:100", "赵六").await;

    // 异步获取
    if let Some(name) = cache.get(&"user:100").await {
        println!("异步获取缓存：{}", name); // 输出：异步获取缓存：赵六
    }

    // 异步删除
    cache.invalidate(&"user:100").await;

    // 异步原子填充：get_with（不存在则计算并插入，避免并发重复计算）
    // 场景：缓存不存在时，从数据库查询并插入缓存，这里模拟耗时操作
    let user = cache
        .get_with("user:101", async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            "孙七"
        })
        .await;
    println!("原子填充缓存：{}", user); // 输出：原子填充缓存：孙七
}

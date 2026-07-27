`Rust` 的 `match` 表达式功能全面，但在只关心一种匹配情况时，
写一个包含 `_ => {}` 的完整 `match` 显得非常啰嗦，增加了无谓的缩进层级。

> 使用 `if let` 处理单次匹配，`while let` 处理迭代器或流的循环匹配。

```
fn main() {
    // 场景：只处理配置存在的情况
    let app_mode: Option<&str> = Some("Production");

    // 这种写法比 match 更加扁平、直观
    if let Some(mode) = app_mode {
        println!("当前运行模式: {}", mode);
    }

    // 场景：从消费队列中不断取出数据直到为空
    let mut tasks = vec!["Task A", "Task B", "Task C"].into_iter();

    // 只要 next() 返回 Some，循环就继续
    while let Some(task) = tasks.next() {
        println!("正在处理: {}", task);
    }
}
```

> 来源：[掘金 ServBay](https://juejin.cn/post/7587020682010591259)

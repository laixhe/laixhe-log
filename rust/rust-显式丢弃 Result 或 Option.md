`Rust` 的 `Result` 和 `Option` 类型强制开发者处理潜在的错误或空值。

> 在某些场景下，比如发送一个非关键的统计与日志数据，即使失败了也无伤大雅。

```
use std::fs;

fn main() {
    // 场景：尝试删除一个可能存在的临时缓存文件
    let _ = fs::remove_file("/tmp/temp.txt"); 

    // 场景：持有一个 Option 数据，显式调用 drop 立即释放，不再占用作用域后续的资源
    let config_data: Option<String> = Some(String::from("Heavy Config Data"));
    drop(config_data); 
    
    // 此时再访问 config_data 会导致编译错误，因为所有权已移交并销毁
}
```

> 来源：[掘金 ServBay](https://juejin.cn/post/7587020682010591259)

很多开发者习惯用 `Vec` 处理所有列表数据。但在需要频繁从头部删除数据（FIFO队列）的场景下，`Vec` 的性能其实不是很好。
因为 Vec::remove(0) 会导致后续所有元素向前移动，时间复杂度是 O(n)

> 遇到队列需求，直接使用 `VecDeque`。它基于环形缓冲区实现，头部和尾部的操作都是 O(1)

```
use std::collections::VecDeque;

fn main() {
    // 初始化一个双端队列
    let mut buffer = VecDeque::from(vec![100, 200, 300]);

    // 从头部弹出元素，对于大量数据，这比 Vec 快几个数量级
    if let Some(val) = buffer.pop_front() {
        println!("处理队首数据: {}", val);
    }

    // 依然可以在尾部添加
    buffer.push_back(400);
}
```

> 来源：[掘金 ServBay](https://juejin.cn/post/7587020682010591259)

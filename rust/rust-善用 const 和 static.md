在 `Rust` 中定义全局值，新手容易混淆 `const` 和 `static`

- `const` 编译时常量。编译器会在用到它的地方直接将其内联（复制），它不占用运行时的固定内存地址
- `static` 静态变量。它在整个程序生命周期内拥有固定的内存地址

> 数值计算、配置参数用 `const`
>
> 需要全局唯一地址或配合原子操作（Atomic）做全局状态管理时用 `static`

```
use std::sync::atomic::{AtomicUsize, Ordering};

// 编译时替换，哪里用到 MAX_Connections，哪里就变成 100
const MAX_CONNECTIONS: u32 = 100; 

// 全局唯一的计数器，拥有固定内存地址
static ACTIVE_USERS: AtomicUsize = AtomicUsize::new(0);

fn new_connection() {
    // 增加全局计数
    ACTIVE_USERS.fetch_add(1, Ordering::SeqCst);
    
    // 使用常量做逻辑判断
    if ACTIVE_USERS.load(Ordering::SeqCst) as u32 <= MAX_CONNECTIONS {
        println!("允许连接");
    }
}
```

> 来源：[掘金 ServBay](https://juejin.cn/post/7587020682010591259)

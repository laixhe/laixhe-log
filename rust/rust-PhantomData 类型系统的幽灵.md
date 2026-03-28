`PhantomData` 是一个零大小类型（Zero-sized Type），它不占用任何运行时内存。
它的存在纯粹是为了欺骗编译器，让编译器认为结构体拥有某种数据的所有权或生命周期关系。

- 这种幽灵数据是实现零成本抽象（Zero-cost Abstractions）的核心工具之一

> 当定义一个泛型结构体，但该泛型参数实际上并不作为字段存储时（例如用于状态标记或 FFI 边界），使用 `PhantomData` 避免编译错误。



```
use std::marker::PhantomData;

// 定义两种状态类型
struct Connected;
struct Disconnected;

// 这是一个带有状态标记的客户端结构体
// T 只是用来在编译期区分状态，不占用运行时空间
struct Client<T> {
    id: u32,
    _state: PhantomData<T>, 
}

impl<T> Client<T> {
    fn new(id: u32) -> Self {
        Client {
            id,
            _state: PhantomData,
        }
    }
}

fn main() {
    let c1: Client<Connected> = Client::new(1);
    let c2: Client<Disconnected> = Client::new(2);

    // 编译器会认为 c1 和 c2 是完全不同的类型
    // 防止了错误地将断开连接的客户端传入需要连接状态的函数中
    println!("客户端ID: {}", c1.id);
}
```

> 来源：[掘金 ServBay](https://juejin.cn/post/7587020682010591259)

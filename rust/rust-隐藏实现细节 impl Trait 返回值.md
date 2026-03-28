编写库或 `API` 时，如果返回类型非常复杂（例如一长串的迭代器链 `Map<Filter<...>>`），不仅写起来痛苦，维护也是噩梦。一旦内部实现变动，函数签名就得改。

> 使用 `-> impl Trait` 这就是告诉调用者：返回一个实现了这个特质的东西，具体类型不需要知道。

```
// 不需要写出具体的返回类型，比如 std::iter::Map<std::ops::Range<...>>
// 只要它能迭代出 u32 即可
fn get_odd_numbers(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|x| x % 2 != 0)
}

fn main() {
    let odds = get_odd_numbers(10);
    
    // 调用者只把它当迭代器用，完全解耦了具体实现
    for num in odds {
        println!("奇数: {}", num);
    }
}
```

这种不透明的返回类型极大地提高了API的稳定性和灵活性。

> 来源：[掘金 ServBay](https://juejin.cn/post/7587020682010591259)

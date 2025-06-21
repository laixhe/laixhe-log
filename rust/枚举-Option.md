##### Option 类型
- 是 Rust 标准库中的一个枚举类型

```
enum Option<T> {
    Some(T),
    None,
}
```

- 常用方法
  - is_some()：判断 Option 是否包含值，返回一个布尔值
  - is_none()：判断 Option 是否不包含值，返回一个布尔值
  - unwrap()：获取 Option 中的值，如果 Option 是 Some，则返回值；如果 Option 是 None，则触发 panic
  - unwrap_or(default)：获取 Option 中的值，如果 Option 是 Some，则返回值；如果 Option 是 None，则返回指定的默认值
  - expect(msg)：获取 Option 中的值，如果 Option 是 Some，则返回值；如果 Option 是 None，则触发 panic，并显示指定的错误消息


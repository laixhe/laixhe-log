### 匹配 match
- 类似其他语言的 switch 语句

```
# 基本用法
fn main() {
    let number = 3;

    // 单个匹配
    match number {
        1 => println!("11"),
        2 => println!("22"),
        3 => println!("33"), // 相当于 switch case
        _ => println!("00"), // 相当于 switch default
    }

    // 多个匹配
    match number {
        1 | 2 => println!("11 22"),
        3 | 4 => println!("33 44"),
        _ => println!("00"),
    }

}

# 匹配枚举类型
enum Message {
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Move { x, y } => println!("Move: (x={}, y={})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("ChangeColor: (r={}, g={}, b={})", r, g, b),
    }
}

fn main() {
    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write(String::from("Hello, world..."));
    let msg3 = Message::ChangeColor(255, 0, 0);

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
}


# 匹配枚举类型(使用 if let 简化匹配)
enum Value {
    Number(i32),
    Text(String),
}

fn main() {
    let value = Value::Number(42);

    // 表达式判断一个 Value 变量是否是 Number 类型，而不需要处理其他类型
    if let Value::Number(n) = value {
        println!("The value is a number: {}", n);
    } else {
        println!("The value is not a number");
    }
}

```

//! 控制流：if / for / while / loop / match / if let。
//!
//! ## 前置知识
//! - `if` 是表达式，可返回值（对应 Python 三元 / C# ?:）
//! - 循环：`loop`（无限循环）、`while`（条件循环）、`for`（迭代）
//! - `match` 是 Rust 最强模式匹配（对应 Go switch / Python match）
//!
//! ## 练习题
//! 1. 用 `match` 把数字 1-3 转成中文一二三，其余返回"其他"。
//! 2. 用 `loop` + `break` 求 1 到 100 的和。

// ============ if 表达式 ============
pub fn if_else() {
    let score = 85;

    // if 是表达式：可以返回值（对应 Python 三元表达式）
    let grade = if score >= 90 {
        "A"
    } else if score >= 60 {
        "B"
    } else {
        "C"
    };
    println!("分数 {} 等级 {}", score, grade); // 85 B
}

// ============ 循环 ============
pub fn loops() {
    // for：遍历范围 / 集合（对应 Python for / C# foreach）
    println!("for 1..3:");
    for i in 1..4 {
        println!("  {}", i); // 1 2 3
    }

    // for 遍历数组
    let arr = [10, 20, 30];
    for v in arr {
        println!("数组元素: {}", v);
    }

    // while：条件循环（对应 Go while 等价）
    let mut n = 3;
    while n > 0 {
        print!("{} ", n);
        n -= 1;
    }
    println!();

    // loop：无限循环，用 break 退出（对应 C for(;;) / Go for {}）
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // break 可带返回值（loop 表达式的值）
        }
    };
    println!("loop break 返回值: {}", result); // 20

    // continue：跳过本次迭代
    for i in 1..=6 {
        if i % 2 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!(); // 1 3 5

    // 循环标签：break 跳出外层循环（对应 Go 标号 break）
    'outer: for i in 1..3 {
        for j in 1..3 {
            if j == 2 {
                break 'outer;
            }
            println!("i={} j={}", i, j);
        }
    }
}

// ============ match 模式匹配 ============
pub fn match_pattern() {
    let day = "周三";

    // match 多分支（对应 Go switch；Rust 的 match 必须穷尽所有可能）
    let kind = match day {
        "周一" | "周二" | "周三" => "工作日",
        "周六" | "周日" => "休息日",
        _ => "未知", // 通配符：兜底分支（对应 default）
    };
    println!("{} -> {}", day, kind);

    // match 匹配数字
    let num = 7;
    let desc = match num {
        1 => "one",
        2 | 3 => "two or three", // 多模式
        4..=6 => "four to six",  // 范围模式
        _ => "many",             // 兜底
    };
    println!("{} -> {}", num, desc);

    // if let：只关心一种情况的简化 match（对应 Go 单 case switch）
    let maybe: Option<i32> = Some(42);
    if let Some(v) = maybe {
        println!("if let 解构: {}", v);
    }
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    // 练习 1：match 数字转中文
    #[test]
    fn exercise_1_match_number() {
        let to_cn = |n: i32| match n {
            1 => "一",
            2 => "二",
            3 => "三",
            _ => "其他",
        };
        assert_eq!(to_cn(1), "一");
        assert_eq!(to_cn(9), "其他");
    }

    // 练习 2：loop + break 求 1..=100 的和
    #[test]
    fn exercise_2_loop_sum() {
        let mut sum = 0;
        let mut i = 1;
        let result = loop {
            sum += i;
            i += 1;
            if i > 100 {
                break sum;
            }
        };
        assert_eq!(result, 5050);
    }
}

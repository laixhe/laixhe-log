//! ⭐ 迭代器专题：Rust 函数式编程的灵魂。
//!
//! ## 前置知识
//! **迭代器（Iterator）** 是一种「能依次产生一系列元素」的惰性抽象。
//! Rust 中 `for` 循环本质就是语法糖——自动调用 `into_iter()` 然后不断 `next()`。
//!
//! 三类转化（按 self 接收者区分）：
//! - `iter()`      → 产生 `&T`       （只借用，原集合还能用）
//! - `iter_mut()`  → 产生 `&mut T`   （可变借用，能改元素）
//! - `into_iter()` → 产生 `T`         （拿走所有权，原集合被消耗）
//!
//! 两类适配器：
//! - **迭代器适配器**（Iterator Adapter）：map / filter / take / skip 等，**返回新的迭代器**，惰性执行
//! - **消费适配器**（Consumer / Sink）：collect / sum / for_each / fold 等，**真正触发迭代**，产生结果
//!
//! ## 练习题
//! 1. 用一行迭代器链把 `vec![1,2,3,4,5]` 中所有偶数平方后求和。
//! 2. 用 `zip` 和 `enumerate` 对比它们生成的索引有什么不同。
//! 3. 用 `flat_map` 把 `vec!["hello", "world"]` 中每个单词的字符逐个展开成字符序列。

// ============ 基础：迭代器的三种遍历方式 ============
pub fn basics() {
    let v = vec![10, 20, 30];

    // 1) iter()：只读借用（最常用）
    print!("iter(): ");
    for x in v.iter() {         // x 的类型是 &i32
        print!("{x} ");
    }
    println!();

    // 2) iter_mut()：可变借用（能改元素）
    let mut v2 = vec![1, 2, 3];
    for x in v2.iter_mut() {    // x 的类型是 &mut i32
        *x *= 2;                // 解引用后修改
    }
    println!("iter_mut() 加倍后: {:?}", v2); // [2,4,6]

    // 3) into_iter()：拿走所有权（for 循环默认调用这个）
    print!("into_iter(): ");
    for x in v {                // x 的类型是 i32（所有权转移）
        print!("{x} ");
    }
    println!();
    // println!("{:?}", v);     // ❌ 编译错误！v 已经被 move 了

    // 手动调用 next()（展示迭代器底层）
    let v3 = vec!['a', 'b', 'c'];
    let mut it = v3.iter();
    println!("手动 next: {:?} {:?} {:?} {:?}", it.next(), it.next(), it.next(), it.next());
    // Some('a') Some('b') Some('c') None
}

// ============ 迭代器适配器（返回新迭代器，惰性！）============
//
// 这些函数都**不会立刻执行**，只是在原迭代器外包了一层，直到消费适配器触发才真正计算。
pub fn adapters() {
    let nums = 1..=10; // Range 本身就是迭代器（1..=10 表示 1 到 10，包含两端）

    // ---- map：对每个元素做变换 ----
    let squares: Vec<i32> = nums.map(|x| x * x).collect();
    println!("1..=10 平方: {:?}", squares); // [1,4,9,16,25,36,49,64,81,100]

    // ---- filter：只保留满足条件的元素 ----
    let words = vec!["rust", "go", "python", "java", "c++", "js"];
    let short_words: Vec<_> = words.iter().filter(|w| w.len() <= 3).collect();
    println!("长度≤3 的单词: {:?}", short_words); // ["go", "c++", "js"]

    // ---- filter_map：filter + map 二合一，闭包返回 Option，None 会被过滤 ----
    // 场景：把字符串数组里「能成功解析为数字」的挑出来并转成 i32
    let strs = vec!["123", "abc", "456", "not_a_num", "789"];
    let nums: Vec<i32> = strs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok()) // parse 成功返回 Some(n)，失败返回 None 被过滤
        .collect();
    println!("filter_map 选出合法数字: {:?}", nums); // [123, 456, 789]

    // ---- take(n) / skip(n)：取前 n 个 / 跳过前 n 个 ----
    let r: Vec<_> = (1..=10).take(3).collect();
    println!("take(3): {:?}", r); // [1,2,3]
    let r: Vec<_> = (1..=10).skip(7).collect();
    println!("skip(7): {:?}", r); // [8,9,10]

    // ---- step_by(n)：每 n 个取一个 ----
    let r: Vec<_> = (0..=20).step_by(5).collect();
    println!("step_by(5) 0..=20: {:?}", r); // [0,5,10,15,20]

    // ---- enumerate：给每个元素加上索引（从 0 开始）----
    for (i, ch) in "Rust".chars().enumerate() {
        println!("  enumerate: [{i}] = '{ch}'"); // [0]='R' [1]='u' [2]='s' [3]='t'
    }

    // ---- zip：把两个迭代器的元素一一配对（长度以较短的为准）----
    let names  = ["Alice", "Bob", "Charlie"];
    let scores = [95,          87,    92];
    let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("zip 配对: {:?}", pairs); // [("Alice",95), ("Bob",87), ("Charlie",92)]

    // ---- chain：把两个迭代器首尾相接 ----
    let a = 1..=3;
    let b = 10..=12;
    let c: Vec<_> = a.chain(b).collect();
    println!("chain: {:?}", c); // [1,2,3,10,11,12]

    // ---- flatten：把嵌套的迭代器展平一层 ----
    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    let flat: Vec<_> = nested.iter().flatten().collect();
    println!("flatten: {:?}", flat); // [1,2,3,4,5,6]

    // ---- flat_map：flatten + map 二合一 ----
    // 场景：把每个单词的字符展开
    let words2 = vec!["hello", "world"];
    let chars: Vec<_> = words2.iter().flat_map(|w| w.chars()).collect();
    println!("flat_map 展开字符: {:?}", chars); // ['h','e','l','l','o','w','o','r','l','d']
}

// ============ 消费适配器（真正触发计算）============
pub fn consumers() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // ---- collect：把迭代器元素收集到目标集合（最常用的消费适配器）----
    // 需要显式指定目标类型（通过变量类型声明 或 turbofish ::<Vec<_>>）
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("collect 到 Vec: {:?}", doubled);

    // 也可以收集到 HashSet / BTreeMap 等（只要实现了 FromIterator trait）
    use std::collections::HashSet;
    let unique: HashSet<_> = v.iter().collect();
    println!("collect 到 HashSet（去重）: {:?}", unique);

    // ---- sum / product：求和 / 乘积 ----
    let s: i32 = v.iter().sum();
    let p: i32 = v.iter().product();
    println!("sum={s}, product={p}"); // sum=31, product=6480

    // ---- count：统计元素个数 ----
    // 闭包参数：v.iter() 产出 &&i32，经过模式匹配 &&x 把 x 解成 &i32；
    // 又因为 i32 是 Copy，&i32 参与算术运算时自动 deref，所以直接 x % 2 就能用。
    let even_cnt = v.iter().filter(|&&x| x % 2 == 0).count();
    println!("偶数个数={even_cnt}"); // 3（4, 2, 6）

    // ---- min / max：最小 / 最大值（返回 Option，空迭代器是 None）----
    println!("min={:?} max={:?}", v.iter().min(), v.iter().max()); // Some(1) Some(9)

    // ---- any / all：是否「有一个」/「全部」满足条件 ----
    println!("any > 10? {}", v.iter().any(|&x| x > 10));  // false
    println!("all > 0?  {}", v.iter().all(|&x| x > 0));   // true

    // ---- for_each：对每个元素执行副作用（替代 for 循环的函数式写法）----
    print!("for_each: ");
    (1..=5).for_each(|x| print!("{x} "));
    println!();

    // ---- fold / reduce：累积聚合 ----
    // fold(初始值, |累积器, 元素| 新的累积值) —— 始终返回初始值类型
    let sum_fold: i32 = (1..=10).fold(0, |acc, x| acc + x);
    println!("fold 累加 1..=10 = {}", sum_fold); // 55

    // reduce：没有初始值，用第一个元素作起点，返回 Option（空迭代器返回 None）
    let max_reduce = v.iter().reduce(|a, b| if a > b { a } else { b });
    println!("reduce 手动求最大值 = {:?}", max_reduce); // Some(9)

    // ---- partition：按条件分成两个集合 ----
    let (even, odd): (Vec<i32>, Vec<i32>) = v.iter().partition(|&&x| x % 2 == 0);
    println!("partition 奇偶分: 偶={:?}  奇={:?}", even, odd);
}

// ============ 综合实战：用迭代器处理复杂查询 ============
//
// 场景：给定一批员工（部门，年龄，月薪），用迭代器链求出「R&D 部门 30 岁以上员工的平均月薪」。
pub fn practice() {
    struct Employee {
        dept:   &'static str,
        age:    u32,
        salary: u32,
    }

    let staff = vec![
        Employee { dept: "R&D",  age: 28, salary: 30000 },
        Employee { dept: "R&D",  age: 35, salary: 45000 },
        Employee { dept: "R&D",  age: 42, salary: 60000 },
        Employee { dept: "HR",   age: 32, salary: 18000 },
        Employee { dept: "R&D",  age: 25, salary: 22000 },
        Employee { dept: "Sale", age: 38, salary: 25000 },
    ];

    // 要求：R&D 部门 + 30 岁以上 → 平均月薪
    let query = staff.iter()
        .filter(|e| e.dept == "R&D")        // 先筛选部门
        .filter(|e| e.age >= 30)            // 再筛选年龄
        .map(|e| e.salary as f64)           // 提取月薪转浮点
        .collect::<Vec<_>>();               // 收集（下面要两次用：sum + len）

    let avg = if query.is_empty() {
        0.0
    } else {
        let total: f64 = query.iter().sum();
        total / query.len() as f64
    };
    println!("R&D 30+ 员工平均月薪: {:.0} 元/月", avg); // (45000+60000)/2 = 52500
}

// ============ 练习题参考答案（cargo test 可验证）============
#[cfg(test)]
mod tests {
    // 练习 1：偶数平方后求和（1..=5 → 2^2 + 4^2）
    #[test]
    fn exercise_1_even_squares_sum() {
        let sum: i32 = (1..=5).filter(|x| x % 2 == 0).map(|x| x * x).sum();
        assert_eq!(sum, 20);
    }

    // 练习 2：zip 与 enumerate 的索引区别
    #[test]
    fn exercise_2_zip_vs_enumerate() {
        let v = vec!['a', 'b', 'c'];
        // enumerate：索引固定从 0 开始
        let e: Vec<_> = v.iter().enumerate().collect();
        assert_eq!(e, vec![(0, &'a'), (1, &'b'), (2, &'c')]);
        // zip：可以和任意迭代器配对，这里是 100 开头的序列
        let z: Vec<_> = (100..).zip(v.iter()).collect();
        assert_eq!(z, vec![(100, &'a'), (101, &'b'), (102, &'c')]);
    }

    // 练习 3：flat_map 把每个单词展开成字符
    #[test]
    fn exercise_3_flat_map_chars() {
        let words = vec!["hello", "world"];
        let chars: Vec<char> = words.iter().flat_map(|w| w.chars()).collect();
        assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']);
    }
}

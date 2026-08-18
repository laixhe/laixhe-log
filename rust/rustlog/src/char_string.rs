//! 字符与字符串类型示例：char、String、&str 的常用操作、遍历与类型转换。
//!
//! ## 前置知识
//! - **`char`**：Unicode 标量值（不是字节！），4 字节固定大小。能表示中文、Emoji 等。
//! - **`String`**：可增长、可修改、拥有所有权的 UTF-8 字符串，在堆上分配内存。
//! - **`&str`**：字符串切片（String Slice），对某段 UTF-8 字节的**引用**，不拥有所有权。
//!   字符串字面量 `"hello"` 的类型就是 `&'static str`（静态生命周期，整个程序期间有效）。
//!
//! 简单记忆：
//! - `&str` 像「借来的书」，只能看，不能改；
//! - `String` 像「自己买的书」，可以写画、改封面、撕掉一页。
//!
//! ## 练习题
//! 1. 把 `"  Hello,Rust!  "` 首尾空格去掉，再把逗号替换成空格，再按空格分割为单词。
//! 2. 用 `matches!` 判断一个 `char` 是否为中文（提示：`'一'..='鿿'` 是 CJK 统一汉字
//!    的 Unicode 范围，U+4E00 ~ U+9FFF，涵盖了绝大多数常用中文）。
//! 3. 用 `format!` 拼接一个 `&str` 和一个 `String`，观察是否发生所有权转移。

use std::collections::HashMap;

// ============ 字符类型 char ============
// Rust 的 char 类型代表一个 Unicode 标量值，
// 占用 4 个字节。它可以表示中文、表情符号等。
pub fn std_char() {
    let c1: char = 'A';   // 英文字母
    let c2: char = '中';  // 中文字符
    let c3: char = '😀';  // 表情符号（Emoji）
    let c4: char = '1';   // 数字字符
    let c5: char = '!';   // 标点符号

    println!(
        "c1 = {} c2 = {} c3 = {} c4 = {} c5 = {}",
        c1, c2, c3, c4, c5
    );

    // char 的常用判断方法
    println!("'A' 是字母？{}", 'A'.is_alphabetic());   // true
    println!("'1' 是数字？{}", '1'.is_numeric());      // true
    println!("' ' 是空白？{}", ' '.is_whitespace());   // true
    println!("'A' 转小写：{}", 'A'.to_ascii_lowercase()); // a
    println!("'a' 转大写：{}", 'a'.to_ascii_uppercase()); // A
    // 注意：to_lowercase() / to_uppercase() 返回迭代器（德语 ß 转大写是 SS，不是一个字符）
    let mut up = 'ß'.to_uppercase();
    println!("'ß' 转大写：{}{}", up.next().unwrap(), up.next().unwrap());
    // 免责：unwrap() 仅示例方便，生产请用 match/?
}

// ============ String 常用操作 ============
// String 是一个可增长、可修改、拥有所有权的 UTF-8 字符串类型
pub fn std_string() {
    // ---------- 创建 String 的多种方式 ----------
    {
        // 1. 从字面值创建
        let s1 = String::from("hello");
        println!("s1 = {}", s1); // 结果：hello

        // 2. 从字符迭代器创建
        //    注意：String::from_iter 是较新版本才有的「固有方法」；
        //    等价写法（任何版本都可用）：['h','e','l','l','o'].into_iter().collect::<String>()
        let s2 = String::from_iter(['h', 'e', 'l', 'l', 'o']);
        println!("s2 = {}", s2); // 结果：hello

        // 3. to_string()（最常用）
        let mut s3 = "hello".to_string();
        s3.push(' ');       // 追加一个字符（空格）
        s3.push_str("world");  // 追加字符串切片
        println!("s3 = {}", s3); // 结果：hello world

        // 4. format! 宏拼接：返回新的 String，**不会**取得参数的所有权
        let s4 = format!("{}...", s3);
        println!("s4 = {}", s4); // 结果：hello world...
        // s3 仍然可以使用（format! 只借用了 &s3）
        println!("s3 still alive: {}", s3);

        // 5. 使用 + 拼接（右边参数自动解引用为 &str，左边会被消耗掉）
        let a = String::from("Hello, ");
        let b = "world";
        let c = a + b;          // 这里 a 的所有权已经转移，后面不能再用 a
        println!("+ 拼接: {}", c); // Hello, world
        // println!("{}", a);   // ❌ 编译错误！a 已经被 move 了
    }

    // ---------- 长度、字符、字节 ----------
    // len() 返回的是 UTF-8 字节长度，**不是**字符数量！
    {
        let text = String::from("你好");
        println!("len(字节)={}", text.len());                 // 结果：6（一个中文通常 3 字节）
        println!("chars(字符数)={}", text.chars().count());   // 结果：2
        println!("bytes(字节数)={}", text.bytes().count());   // 结果：6

        // 遍历每个字符（Unicode 标量值）
        for character in text.chars() {
            println!("character={}", character);
        }
        // 遍历每个字节（底层 UTF-8 编码）
        for byte in text.bytes() {
            println!("byte={}", byte);
        }
    }

    // ---------- 🌟 补全之前的 TODO：contains / starts_with / ends_with / trim / replace / split ----------
    {
        let s = "   Hello, Rust! I love Rust.   ";

        // 1. contains：判断是否包含子串
        println!("包含 'Rust'？{}", s.contains("Rust"));          // true
        println!("包含 'Python'？{}", s.contains("Python"));      // false

        // 2. starts_with / ends_with：判断前缀 / 后缀
        println!("以 '   He' 开头？{}", s.starts_with("   He"));  // true
        println!("以 '.   ' 结尾？{}", s.ends_with(".   "));      // true

        // 3. trim 家族：去除首尾空白
        println!("trim()      = |{}|", s.trim());      // 去首尾
        println!("trim_start()= |{}|", s.trim_start());// 仅开头
        println!("trim_end()  = |{}|", s.trim_end());  // 仅结尾

        // 4. replace：替换（返回新 String，不修改原字符串）
        let replaced = s.replace("Rust", "🦀 Rust");
        println!("replace: {}", replaced); // Hello, 🦀 Rust! I love 🦀 Rust.

        // 5. split：分割（返回迭代器，可链式调用 collect）
        //    有多种变体：split_whitespace（按任意空白）、split_terminator、splitn 等
        let csv = "apple,banana,cherry,date";
        let fruits: Vec<&str> = csv.split(',').collect();
        println!("split by ',' → {:?}", fruits);
        // 结果：["apple", "banana", "cherry", "date"]
    }

    // ---------- 单词频率统计（综合实战：entry API + 迭代器）----------
    {
        let text = "rust go rust php rust go python js";
        let mut counts = HashMap::new();
        for word in text.split_whitespace() {
            // entry(key)：存在则返回 Entry::Occupied，不存在则插入默认值并返回 Entry::Vacant
            // or_insert(0)：若 key 不存在则插入 0，并返回 &mut i32 可变引用
            let count = counts.entry(word).or_insert(0);
            // *count += 1：通过解引用修改 HashMap 中的值
            *count += 1;
        }
        println!("单词出现次数统计: {:?}", counts);
        // 结果：{"js": 1, "go": 2, "php": 1, "python": 1, "rust": 3}
    }
}

// ============ String vs &str 深入对比 ============
//
// 这是 Rust 新手最容易困惑的概念，单独拎出来讲。
// 核心区别：**所有权**。
pub fn string_vs_str() {
    // --- 场景 1：作为函数参数 ---
    // 函数参数尽量用 &str，而不是 &String。
    // 原因：&str 可以同时接受 String（自动解引用）、字符串字面量、切片，通用性更强。
    fn print_hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let owned = String::from("World");  // String：拥有所有权
    let slice: &str = "Rustaceans";     // &str：只是引用

    print_hello(&owned);    // ✅ &String 自动 coerce 成 &str（ Deref 强制转换）
    print_hello(slice);     // ✅ 直接传 &str
    print_hello("literal"); // ✅ 字面量也是 &str

    // --- 场景 2：&str 只是借用，不能超出原数据生命周期 ---
    let s = String::from("Hello");
    let part: &str = &s[0..3]; // 切片引用 s 的一部分
    println!("part = {}", part); // Hel
    // 如果此时 drop(s)，part 就会悬垂——Rust 编译器会阻止这种情况！
    //
    // ❌ 下面这个「返回悬垂引用」的函数无法通过编译（已注释掉），可取消注释看报错：
    // fn dangling() -> &str {
    //     let s = String::from("Hello");
    //     &s[0..3] // s 在函数结束时被 drop，返回的引用将指向已释放内存
    // }

    // --- 场景 3：什么时候返回 String，什么时候返回 &str？---
    // - 返回 &str：当你只是把**已有**字符串的某一部分借出去（比如 getter）
    // - 返回 String：当函数内部新建了字符串（比如拼接、格式化），必须把所有权交给调用者
}

// ============ 字符串解析为数值类型 ============
// parse::<T>() 通过类型推断或 turbofish 语法指定目标类型
pub fn string_parse() {
    // 转整数
    let i_str = String::from("666");
    let i_res = i_str.parse::<i32>();
    match i_res {
        Ok(i) => println!("i={}", i),
        Err(err) => println!("i err={}", err),
    }

    // 转浮点
    let f_str = String::from("88.88");
    let f_res = f_str.parse::<f64>();
    match f_res {
        Ok(f) => println!("f={}", f),
        Err(err) => println!("f err={}", err),
    }

    // 解析失败的演示：字符串不是合法数值
    let bad = "not_a_number";
    match bad.parse::<i32>() {
        Ok(_) => println!("不可能成功"),
        Err(e) => println!("解析失败演示: '{}' → 错误: {}", bad, e),
    }
    // 生产代码中，应该用 match 或 ? 处理，而不是 unwrap()
}

// ============ 练习题参考答案（cargo test 可验证）============
#[cfg(test)]
mod tests {
    // 练习 1：trim + 替换逗号为空格 + 按空格切词
    #[test]
    fn exercise_1_trim_replace_split() {
        let s = "  Hello,Rust!  ";
        let replaced = s.trim().replace(',', " "); // 先绑定，避免临时值被 drop
        let words: Vec<&str> = replaced.split_whitespace().collect();
        assert_eq!(words, vec!["Hello", "Rust!"]);
    }

    // 练习 2：用 matches! 判断是否为中文（CJK 统一汉字区间）
    #[test]
    fn exercise_2_is_chinese_char() {
        assert!(matches!('中', '一'..='鿿'));
        assert!(!matches!('A', '一'..='鿿'));
    }

    // 练习 3：format! 拼接 &str 和 String（不转移所有权）
    #[test]
    fn exercise_3_format_borrows() {
        let slice: &str = "Hello";
        let owned = String::from(" Rust");
        let combined = format!("{}{}", slice, owned);
        assert_eq!(combined, "Hello Rust");
        // format! 只借用，owned 仍可用
        assert_eq!(owned, " Rust");
    }
}

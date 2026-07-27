use std::collections::HashMap;

// 字符类型 Character
// Rust 的 char 类型代表一个 Unicode 标量值，
// 占用 4 个字节。它可以表示中文、表情符号等。
pub fn std_char() {
    let c1: char = 'A'; // 英文字母
    let c2: char = '中'; // 中文字符
    let c3: char = '😀'; // 表情符号（Emoji）
    let c4: char = '1'; // 数字字符
    let c5: char = '!'; // 标点符号

    println!(
        "c1 = {} c2 = {} c3 = {} c4 = {} c5 = {}",
        c1, c2, c3, c4, c5
    );
}

pub fn std_string() {
    // String 是一个可增长、可修改、拥有所有权的 UTF-8 字符串类型

    // 创建 String 实例
    {
        // 1. 从字面值创建
        let s1 = String::from("hello");
        println!("s1 = {}", s1); // 结果：hello

        // 2. 从字符创建
        let s2 = String::from_iter(['h', 'e', 'l', 'l', 'o']);
        println!("s2 = {}", s2); // 结果：hello

        // 3. 从其他类型创建
        let mut s3 = "hello".to_string();
        println!("s3 = {}", s3); // 结果：hello
        s3.push(' '); // 追加一个字符 空格
        s3.push_str("world"); // 追加字符串
        println!("s3 = {}", s3); // 结果：hello world

        // 使用 format! 拼接，会返回新的 String，通常不会取得参数的所有权
        let s4 = format!("{}...", s3);
        println!("s4 = {}", s4); // 结果：hello world...
    }
    // 获取字符串长度 len() 返回的是 UTF-8 字节长度，不是字符数量
    {
        let text = String::from("你好");
        println!("len={}", text.len()); // 结果：6
        println!("chars={}", text.chars().count()); // 结果：2 （统计字符数量，因为一个中文字符通常占 3 个 UTF-8 字节）
        println!("bytes={}", text.bytes().count()); // 结果：6

        // 遍历字符串字符
        for character in text.chars() {
            println!("character={}", character);
        }
        // 遍历字符串字节
        for byte in text.bytes() {
            println!("byte={}", byte);
        }
    }
    // 判断是否包含内容 contains()
    // 判断开头和结尾 starts_with() ends_with()
    // 去除首尾空白 trim()
    // 替换内容 replace()
    // 切割字符串 split()
    {
        let text = "rust go rust php rust go python js";
        let mut counts = HashMap::new();
        for word in text.split_whitespace() {
            let count = counts.entry(word).or_insert(0);
            *count += 1;
        }
        println!("单词出现次数统计: {:?}", counts); 
        // 结果：{"js": 1, "go": 2, "php": 1, "python": 1, "rust": 3}
    }
}

// 转类型
pub fn convert() {
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
}

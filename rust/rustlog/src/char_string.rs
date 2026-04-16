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

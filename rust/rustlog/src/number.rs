// 转类型
pub fn convert() {
    let i = 666;
    let f1 = 88.888;
    let f2 = 88.0;
    // 转字符串
    println!("i={}", i.to_string());
    println!("f1={}", format!("{:.2}", f1)); // 指定精度为两位小数，结果 f1=88.89
    println!("f2={}", format!("{:.2}", f2)); // 结果 f2=88.00
}

// 溢出
pub fn overflow() {
    let x: u8 = 255;
    // 检查加 1 后是否溢出
    match x.checked_add(1) {
        Some(result) => println!("overflow x={}", result),
        None => println!("overflow x=溢出了"),
    }
}

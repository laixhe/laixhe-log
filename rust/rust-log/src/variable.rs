// 不能使用初始化的变量，默认创建的变量是不可变的，可变要用 mut 关键字表示
// 可根据值自行判定变量类型
// 由关键字```static```声明全局变量

// 基本数据类型
// 布尔   bool (true 或者 false)
// 整型   i8 (-128~127) i16 (-32768~32767) i32 (-2147483648~2147483647) i64 (-9223372036854775808~9223372036854775807) u128 
//        u8 (0~255) u16 (0~65535) u32 (0~4294967295) u64 (0~18446744073709551615) u128
//        isize usize 数据类型的大小将来自机器的架构，在  x86 为 32 位，在 x64 为 64 位
//        f32 单精度 f64 双精度
// 字符串 String &str

// 关于 println!
// 基本数据类型，使用 {}
// 其他数据类型，使用 {:?}，可能需要 #[derive(Debug)] 属性
//

pub fn world_hello() {
    let world = "World";
    let i32_a = 11;
    let f64_a = 2.22;
    println!("Hello, {}, {}, {}", world, i32_a, f64_a);
    println!("Hello, {0}, {1}, {2}", world, i32_a, f64_a);
    println!("Hello, {world}, {i32_a}, {f64_a}");
    println!("{}", format!("Hello, {}, {}, {}", world, i32_a, f64_a));
    println!("{}", format!("Hello, {0}, {1}, {2}", world, i32_a, f64_a));
    println!("{}", format!("Hello, {world}, {i32_a}, {f64_a}"));
    // 结果: Hello, World, 11, 2.22

    // 整数默认类型: i32
    // 浮点默认类型: f64
    // i8 i16 i32 i64 u128 u8 u16 u32 u64 u128
    let a: i32 = 0;
    let b: String = String::from("中文");
    let c = true;  // 推断为 bool 类型
    let d = 1.1;   // 推断为 f64 类型
    let e = 2;     // 推断为 i32 类型
    let f = "ABC中文"; // 推断为 &str 类型

    // 结果为：0,中文,true,1.1,2,ABC中文
    println!("{},{},{},{},{},{}", a, b, c, d, e, f)
}
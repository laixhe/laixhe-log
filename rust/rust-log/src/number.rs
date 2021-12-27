use rand::Rng;
use std::cmp::Ordering;

pub fn number_run(){

    // 整型类型
    // i8 (-128~127) i16 (-32768~32767) i32 (-2147483648~2147483647) i64 (-9223372036854775808~9223372036854775807) u128 
    // u8 (0~255) u16 (0~65535) u32 (0~4294967295) u64 (0~18446744073709551615) u128
    // isize usize 数据类型的大小将来自机器的架构，在  x86 为 32 位，在 x64 为 64 位

    // 浮点类型
    // f32 单精度
    // f64 双精度

    // 16进制 0x
    // 8 进制 0o 
    // 2 进制 0b

    // 整数默认类型: i32
    // 浮点默认类型: f64

    // 数字转成字符串
    let int_str = 100;
    println!("数字转成字符串={}", int_str.to_string());

    // 比较数字
    let cmp_a = 100;
    let cmp_b = 101;
    match cmp_a.cmp(&cmp_b) {
        Ordering::Less => println!("比较数字: a < b"),    // 小于
        Ordering::Greater => println!("比较数字: a > b"), // 大于
        Ordering::Equal => println!("比较数字: a = b"),   // 等于
    }

    // 生成 随机数
    let mut rand_rng = rand::thread_rng();
    for i in 0..10 {
        let rand_number_i = rand_rng.gen::<u32>();
        println!("生成 随机数:{}={}", i, rand_number_i);
    }
    for i in 0..10 {
        let rand_number_i = rand_rng.gen_range(1..11); // 随机 1 到 10 之间
        println!("生成 随机数:{}={}", i, rand_number_i);
    }
    
}
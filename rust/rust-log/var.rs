// 不能使用初始化的变量，默认创建的变量是不可变的，可变要用 mut 关键字表示
// 可根据值自行判定变量类型
// 由关键字```static```声明全局变量

fn main() {
    let world = "World";
    println!("Hello, {}", world);
    
    let a: i32 = 0;
    let b: String = String::from("中文");
    let c = true;  // 推断为 bool 类型
    let d = 1.1;   // 推断为 f64 类型
    let e = 2;     // 推断为 i32 类型
    // 结果为：0,中文,true,1.1,2
    println!("{},{},{},{},{}", a, b, c, d, e)
}
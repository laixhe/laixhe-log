// 由关键字```fn```声明
// 没有可选参数
// 没有默认参数值
// 可利用**元组**可以返回多个不同类型的值

// 没有 `return` 语句则使用最后一条语句的结果作为返回值

//

// 无参数，无返回值
fn void(){
    println!("void")
}
// 有参数，多个返回值
fn test(a:i32, b:bool, s: String) -> (i32, bool) {
    println!("{},{},{}", a, b, s);
    return (a, b);
}

fn main() {
    void();                    // 结果： void
    let (t, tis) = test(1, true, String::from("函数")); // 结果： 1,true,函数
    println!("{},{}", t, tis); // 结果： 1,true
    
    // 匿名函数 lambda
    let sum = |a: i32, b: i32| -> i32 {
        a+b
    };
    println!("{}", sum(1,3)); // 结果： 4
    
    // 使用元组形式返回多值不同类型的值
    let (p1, p2) = pow(2);
    println!("{},{}", p1, p2); // 结果： 4 , 8
    let po = pow(3);
    println!("{},{}", po.0, po.1); // 结果： 9,27
}
// 返回元组
fn pow(n: i32) -> (i32, i32) {
    (n * n, n * n * n)
}

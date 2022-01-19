// 由关键字```fn```声明
// 没有可选参数
// 没有默认参数值
// 可利用**元组**可以返回多个不同类型的值

// 没有 `return` 语句则使用最后一条语句的结果作为返回值

// 无参数，无返回值
fn func_void(){
    println!("func_void")
}

// 有参数，多个返回值(返回元组)
fn func_args(a:i32, b:bool, s: String) -> (i32, bool) {
    println!("{},{},{}", a, b, s);
    return (a, b);
}

pub fn func_run() {
    func_void();                      // 结果：func_void
    let (args1, args2) = func_args(1, true, String::from("函数")); // 结果：1,true,函数
    println!("{},{}", args1, args2); // 结果：1,true
    
    // 匿名(闭包)函数 lambda
    let sum = |a: i32, b: i32| -> i32 {
        a+b
    };
    // 相当于 fn sum(a: i32, b: i32) -> i32 {}

    println!("匿名函数:{}", sum(1,3)); // 结果：4
    
}



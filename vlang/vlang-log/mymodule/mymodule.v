module mymodule

// 模块初始化函数
// 如果在模块中定义了 init 函数，当模块被导入时，init 函数会被自动调用
// 同一个模块内只能定义一个 init 函数
fn init() {
    println('mymodule init...')
}

// pub 是函数的访问控制，只有公共的函数才可以被其他模块使用
// 没有 pub 的只能在模块内部使用
pub fn say_hi() {
	println('mymodule say_hi...')
}
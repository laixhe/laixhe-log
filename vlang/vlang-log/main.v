// 定义主模块
module main

// 导入模块
import os
import time

// 导入自定义模块
import mymodule

// 主函数 - 程序运行入口
pub fn main() {
	println('hello world')     // 语句结尾不需要分号
	println(os.args)           // 使用 os 模块的 args 变量
	println(time.now())        // 调用 time 模块的 now 函数

	println('---------')
	mymodule.say_hi()          // 使用自已定义的模块函数

	println('---------')
	mymodule.basic_type()
}

package main

// 由关键字```const```声明常量，没有枚举类型

//

// 定义常量并使用 iota 模拟枚举
const (
	read int = iota
	green
	blue
)

const uid int = 10

// uid =11; // error

package main

import (
	"fmt"
	"strconv"
)

// 数字类型(整型)
//         uint8 (0~255) uint16 (0~65535) uint32 (0~4294967295) uint64 (0~18446744073709551615)
//         int8 (-128~127)  int16 (-32768~32767)  int32 (-2147483648~2147483647)  int64 (-9223372036854775808~9223372036854775807)
//         int uint 数据类型的大小将来自机器的架构，在  x86 为 32 位，在 x64 为 64 位
// 数字类型(浮点型)
//        float32
//        float64

// 16进制 0x
// 8 进制 0o
// 2 进制 0b

func NumberRun() {
	// 数字转成字符串
	intStr := 100
	fmt.Println("数字转成字符串=", strconv.Itoa(intStr))
	fmt.Println("数字转成字符串=", fmt.Sprintf("%d", intStr))

	// 生成 随机数
	// golang_log/math/rand_main 在这个例子中
}

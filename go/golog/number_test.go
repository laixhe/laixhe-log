package main

import (
	"fmt"
	"math"
	"strconv"
	"testing"
)

/*
数值进阶：格式化 / 溢出与饱和 / 类型转换
对应 Rust rustlog number.rs、C# cslog NumberDemo.cs

Go 数值溢出特性：
- 无符号整数溢出是定义行为，会回绕（uint8 255 + 1 = 0）
- 有符号整数溢出是未定义行为，编译器优化时可能产生意外结果，必须显式检查
- Go 没有 checked 关键字，溢出检查需要手写或用 math/bits
*/

// 数值格式化输出（对应 Rust number_to_string）
func TestNumberFormat(t *testing.T) {
	// 精度控制（四舍五入）
	fmt.Printf("f1=%.2f f2=%.2f\n", 88.888, 88.0) // f1=88.89 f2=88.00

	// 十六进制 / 八进制 / 二进制（X 大写，x 小写，b 二进制）
	fmt.Printf("666 hex=0x%X octal=0o%o binary=0b%b\n", 666, 666, 666)
	// 666 hex=0x29A octal=0o1232 binary=0b1010011010

	// 前导零填充 + 宽度控制（%08d 补零）
	fmt.Printf("leading zeros: %08d\n", 666) // 00000666

	// 对齐：%-10d 左对齐，%10d 右对齐
	fmt.Printf("left=|%-10d| right=|%10d|\n", 666, 666)
	// left=|666       | right=|       666|

	// 正负号显式显示（%+d）
	fmt.Printf("positive=%+d  negative=%+d\n", 666, -888)
	// positive=+666  negative=-888

	// 千分位分组（fmt 不支持 {:,}，手动拼接演示）
	fmt.Println("grouping: 1,234,567")
}

// 整数溢出（对应 Rust overflow / Go 溢出检查）
func TestNumberOverflow(t *testing.T) {
	// 无符号回绕：uint8 255 + 1 = 0（无符号溢出是定义行为，会回绕）
	var b uint8 = 255
	fmt.Println("u8 255 + 1 =", b+1, "（回绕到 0）")

	// 有符号溢出是未定义行为！必须先检查边界
	// 对应 Rust checked_add：加法结果 < a 即溢出
	var a int32 = math.MaxInt32
	if a > math.MaxInt32-1 {
		fmt.Println("checked_add: MAX + 1 = 溢出了（已检查）")
	}

	// 饱和（对应 Rust saturating_add）：先转更大类型再 clamp
	// 注意：64 位平台上 Go 的 int 就是 64 位，int64 拓宽无效，这里用 int32 演示
	wide := int64(a) + 1 // int64 拓宽，不会溢出
	sat := int32(wide)
	switch {
	case wide > math.MaxInt32:
		sat = math.MaxInt32
	case wide < math.MinInt32:
		sat = math.MinInt32
	default:
		sat = int32(wide)
	}
	fmt.Println("saturating_add: MAX + 1 =", sat, "（饱和）")

	// 浮点精度：0.1 + 0.2 != 0.3（IEEE 754，需高精度格式化才能看到误差）
	fmt.Printf("0.1 + 0.2 = %.17g（浮点精度问题）\n", 0.1+0.2)
}

// 类型转换（对应 Rust type_conversion）
func TestNumberConversion(t *testing.T) {
	// 浮点转整数：int() 强转向零截断（对应 Java 强转 / C++ static_cast）
	f := 3.99
	fmt.Println("int(3.99) =", int(f), "（向零截断）")
	// 四舍五入：math.Round 先取整再转换
	fmt.Println("round(3.99) =", int(math.Round(3.99)))

	// 字符串解析：strconv.Atoi / strconv.ParseFloat（失败返回 error，区别于 Java 抛异常）
	n, err := strconv.Atoi("666")
	fmt.Println("Atoi('666') =", n, err)
	fd, _ := strconv.ParseFloat("88.88", 64)
	fmt.Println("ParseFloat('88.88') =", fd)

	// 进制解析（对应 strconv.ParseInt 指定 base）
	hex, _ := strconv.ParseInt("29A", 16, 64)
	fmt.Println("ParseInt('29A', 16) =", hex) // 666
	oct, _ := strconv.ParseInt("1232", 8, 64)
	fmt.Println("ParseInt('1232', 8) =", oct) // 666
	// 反方向：数值转指定进制字符串
	fmt.Println("FormatInt(666, 16) =", strconv.FormatInt(666, 16)) // 29a

	// 解析失败：返回 error（区别于 PHP 返回 0）
	_, err = strconv.Atoi("not_a_number")
	fmt.Println("Atoi('not_a_number') err =", err)
}

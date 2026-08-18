package main

import (
	"fmt"
	"testing"
)

/*
控制流：if / for / range / switch（对应 TS basics.test.ts / Python control_flow.py）
*/

// if / else if / else
func TestIf(t *testing.T) {
	score := 85

	// if 可以带初始化语句（作用域仅限 if 块内，对应 Python 3.8 海象表达式）
	if bonus := 10; score+bonus >= 90 {
		fmt.Println("优秀（含加分）")
	} else if score >= 60 {
		fmt.Println("及格")
	} else {
		fmt.Println("不及格")
	}
}

// for 循环（Go 只有 for，没有 while / do-while）
func TestFor(t *testing.T) {
	// 写法1：完整三段式（对应 C/Java for）
	fmt.Print("for i; i<n: ")
	for i := 0; i < 3; i++ {
		fmt.Print(i, " ")
	}
	fmt.Println()

	// 写法2：省略条件 = while（对应 C/Java while）
	sum, i := 0, 0
	for i < 3 { // 等价 for ; i < 3;
		sum += i
		i++
	}
	fmt.Println("while 等价: sum =", sum)

	// 写法3：无限循环（对应 C for(;;) / while(true)）
	count := 0
	for { // 等价 for true
		count++
		if count >= 3 {
			break // 跳出循环
		}
	}
	fmt.Println("无限循环 break:", count)

	// continue：跳过本次循环
	fmt.Print("continue 跳过偶数: ")
	for i := 1; i <= 6; i++ {
		if i%2 == 0 {
			continue
		}
		fmt.Print(i, " ")
	}
	fmt.Println()

	// 循环标号：跳出多层嵌套（对应 Java 标号 break）
BreakOuter:
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if j == 1 {
				break BreakOuter // 直接跳出外层循环
			}
			fmt.Println("i,j =", i, j)
		}
	}
}

// range 遍历（for + range，对应 Python enumerate / C# foreach）
func TestRange(t *testing.T) {
	// 数组 / 切片
	nums := []int{10, 20, 30}
	for i, v := range nums {
		fmt.Println("索引:", i, "值:", v)
	}

	// 忽略索引（_ 占位符，对应 Python _）
	sum := 0
	for _, v := range nums {
		sum += v
	}
	fmt.Println("求和:", sum) // 60

	// 字符串（按 UTF-8 码点遍历）
	for i, r := range "中a" { // 中 占 1 个 rune，索引按字节
		fmt.Printf("rune[%d] = %c\n", i, r)
	}

	// map 的 range 遍历见 map_test.go 的 TestMapRange（键值对顺序随机）
}

// switch（对应 Python match / Java switch，Go 不需要 break 自动跳出）
func TestSwitch(t *testing.T) {
	day := "周三"

	// 基本 switch：每个 case 默认自动 break（区别于 C/Java 需要手动 break）
	switch day {
	case "周一", "周二", "周三", "周四", "周五": // 多值匹配
		fmt.Println("工作日")
	case "周六", "周日":
		fmt.Println("休息日")
	default:
		fmt.Println("未知")
	}

	// 表达式 switch：case 里写条件（对应多 if-else 的简化）
	score := 85
	switch {
	case score >= 90:
		fmt.Println("A")
	case score >= 60:
		fmt.Println("B")
	default:
		fmt.Println("C")
	}

	// fallthrough：强制执行下一个 case（Go 特有，其他语言一般没有）
	n := 2
	switch n {
	case 1:
		fmt.Println("case 1")
		fallthrough // 继续执行 case 2
	case 2:
		fmt.Println("case 2")
	default:
		fmt.Println("default")
	}

	// switch 不带表达式 = if-else 链（类型判断场景见 type_test.go）
}

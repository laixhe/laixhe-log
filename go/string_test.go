package main

import (
	"fmt"
	"strings"
	"testing"
)

/*
Go 源码文件默认采用 Unicode 字符集，Unicode 码点和内存中字节序列的变换实现使用了 UTF-8，在编程无需考虑编码转换的问题
string 是一个 8 位字节的集合，通常但不一定代表 UTF-8编码 的文本，string 可以为空，但是不能为 nil，值是不能改变的
byte 是 uint8 的别名，在所有方面都等同于 uint8 ，用于区分 字节值 和 8位无符号整数值，常用来处理 ASCII 字符
rune 是 int32 的别名，在所有方面都等同于 int32 ，用于区分 字符值 和 整数值，常用来处理 unicode 或 utf-8 字符
rune 存储 Unicode 的码，即一个 中文 占一个 Unicode 码值

字符串实际上是只读的字节切片，对于字符串底层而言就是一个 byte 数组，不过这个数组是只读的，不允许修改

源码文件：runtime/string.go
type stringStruct struct {
    str unsafe.Pointer
    len int
}

*/

// 字符串基本使用
func TestString(t *testing.T) {
	// 分隔字符串
	fmt.Println(strings.Split("1,2,3", ",")) // 结果：[1 2 3]
	// 切割字符串（只切割第一个出现的）
	fmt.Println(strings.Cut("1,2,3", ",")) // 结果：1 2,3 true （返回三个参数）
	// 拼接字符串
	fmt.Println(strings.Join([]string{"a", "b", "c"}, ",")) // 结果：a,b,c
	// 判断字符串开头
	fmt.Println(strings.HasPrefix("开头abc", "开头")) // 结果：true
	// 判断字符串结尾
	fmt.Println(strings.HasSuffix("abc结尾", "结尾")) // 结果：true
	// 查找到第一次出现的位置，索引从 0 开始（-1 没有查找到）
	fmt.Println(strings.Index("查找到第一次出现的位置", "出现")) // 结果：18
	// 查找到最后出现的位置，索引从 0 开始（-1 没有查找到）
	fmt.Println(strings.LastIndex("查找到最后出现的位置", "出现")) // 结果：15

	// 替换字符串（-1 表示替换全部）
	fmt.Println(strings.Replace("aaa替换aaa字符串aaa", "a", "", -1)) // 结果：替换字符串
	// 替换字符串(多个)-使用特定规则(成对出现)对字符串进行替换操作
	fmt.Println(strings.NewReplacer("<", "&lt;", ">", "&gt;").Replace("替换<字符串>规则")) // 结果：替换&lt;字符串&gt;规则

	// 判断字符串是否包含 xxx
	fmt.Println(strings.Contains("判断包含字符串", "包含")) // 结果：true
	// 判断两个字符串是否相等（忽略大小写，同时它还会对特殊字符进行转换）
	fmt.Println(strings.EqualFold("AB大", "ab大")) // 结果：true

	// strings.TrimSpace 去除字符串两侧的空白字符
	// strings.Trim      去除字符串两侧的指定字符串
	// strings.ToLower   转小写
	// strings.ToUpper   转大写
	// strings.Clone     克隆
}

// 字符串处理
func TestStringHandle(t *testing.T) {
	appendBuf := make([]byte, 0, 30)
	appendBuf = fmt.Appendf(appendBuf, "字符串%s处理", "追加") // 零分配（如果 appendBuf 容量充足）
	fmt.Println(string(appendBuf))                      // 结果：字符串追加处理

	fmt.Println(fmt.Sprintf("%s字符串%d", "拼接", 123)) // 结果：拼接字符串123
}

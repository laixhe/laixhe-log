package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

// Go 源码文件默认采用 Unicode 字符集，Unicode 码点和内存中字节序列的变换实现使用了 UTF-8，在编程无需考虑编码转换的问题
// string 是一个 8 位字节的集合，通常但不一定代表 UTF-8编码 的文本，string 可以为空，但是不能为 nil，值是不能改变的
// byte 是 uint8 的别名，在所有方面都等同于 uint8 ，用于区分 字节值 和 8位无符号整数值，常用来处理 ascii 字符
// rune 是 int32 的别名，在所有方面都等同于 int32 ，用于区分 字符值 和 整数值，常用来处理 unicode 或 utf-8 字符
// rune 存储 Unicode 的码，即一个 中文 占一个 Unicode 码值
//
// 字符串实际上是只读的字节切片，对于字符串底层而言就是一个 byte 数组，不过这个数组是只读的，不允许修改
//
//
// 源码文件：runtime/string.go
/**
type stringStruct struct {
    str unsafe.Pointer
    len int
}
*/

func StringRun() {

	fmt.Println("======================================")

	// 拼接字符串
	var hello = "Hello"
	var world string = "World"
	var strInt = 100

	hello0 := hello + "," + world + "," + strconv.Itoa(strInt)
	hello1 := fmt.Sprintf("%s,%s,%d", hello, world, strInt)
	fmt.Println(hello0)
	fmt.Println(hello1)
	// 结果： Hello,World,100

	// 拼接字符串(其他)
	var hello4 = strings.Builder{} // bytes.Buffer 用法也差不多，都是 非线程安全 的有扩容
	hello4.Grow(50)                // 初始化长度(预估容量)
	hello4.WriteString(hello)
	hello4.WriteByte(',')
	hello4.WriteString(world)
	hello4.WriteByte(',')
	hello4.WriteString(strconv.Itoa(strInt))
	var hello5 = hello4.String()
	fmt.Printf("[%s]", hello5)
	// 结果： Hello,World,100
	// 当拼接的字符串多时，strings.Builder 的优势越来越明显

	// 字符串转成数字
	strInt, err := strconv.Atoi("4")
	if err != nil {
		fmt.Println("字符串转成数字失败：", err)
	}
	fmt.Printf("字符串转成数字=%d\n", strInt)

	// 分隔字符串
	var strSplit = "1,2,3,4,5"
	var strSplitArray = strings.Split(strSplit, ",")
	// 注意：即使 strSplit 为空的字符串，在进行分隔后，也会返回 一个长度为 1 的数组
	fmt.Println("分隔字符串1=", strSplitArray, len(strSplitArray))
	// 结果：[1 2 3 4 5]

	// 以空格(空白、换行符)分隔字符串
	strSplit = "1 2    3\n4\n\r5\t6"
	strSplitArray = strings.Fields(strSplit)
	fmt.Println("分隔字符串2=", strSplitArray, len(strSplitArray))
	// 结果：[1 2 3 4 5 6]

	// 拼接字符串(数组)
	strJoin := strings.Join([]string{"a", "b", "c"}, ",")
	fmt.Println("拼接字符串=", strJoin)
	// 结果：a,b,c

	// 去除字符串两侧的空白字符
	strTrimSpace := strings.TrimSpace(" EFG   \n\n\r\t")
	fmt.Println("去除字符串两侧的空白字符=", strTrimSpace)
	// 结果：EFG

	// TrimLeft() 去除字符串左边的空格
	// TrimRight()   去除字符串末尾的空格
	// 去除字符串两侧的指定字符
	strTrim := strings.Trim(",去除字符串,", ",")
	fmt.Println("去除字符串两侧的指定字符=", strTrim)
	// 结果：去除字符串

	// 将字符串中的所有 字母 都转为对应的大写字符
	strUpper := strings.ToUpper("ab字母为大写cdEF")
	fmt.Println("转大写=", strUpper)
	// 结果：AB字母为大写CDEF

	// 将字符串中的所有 字母 都转为对应的小写字符
	strLower := strings.ToLower("AB字母为小写CDef")
	fmt.Println("转小写=", strLower)
	// 结果：ab字母为小写cdef

	// 替换字符串
	strReplace := strings.Replace("aaa替换aaa字符串aaa", "a", "", -1) // -1 表示替换全部
	fmt.Println("替换字符串=", strReplace)
	// 结果：替换字符串

	// 判断字符串是否以 xxx 开头
	strPrefix := strings.HasPrefix("开头abc开头123", "开头")
	fmt.Println("以什么开始=", strPrefix)
	// 结果：true

	// 判断字符串是否以 xxx 结尾
	strSuffix := strings.HasSuffix("以abc结尾123结尾", "结尾")
	fmt.Println("以什么结尾=", strSuffix)
	// 结果：true

	// 查找到第一次出现的位置 (-1表示字符串 str 不包含字符串 substr )，索引从0开始
	strIndex := strings.Index("查找到第一次出现的位置", "出现")
	fmt.Println("第一次出现的位置=", strIndex)
	// 结果：18

	// 查找到最后出现的位置 (-1表示字符串 str 不包含字符串 substr )，索引从0开始
	strLastIndex := strings.LastIndex("查找到最后出现的位置", "出现")
	fmt.Println("最后出现的位置=", strLastIndex)
	// 结果：15

	// 判断字符串是否包含 xxx
	strContains := strings.Contains("判断字符串是否包含", "是否")
	fmt.Println("判断字符串是否包含=", strContains)
	// 结果：true

	// 对比字符串
	// 判断两个字符串是否相等，忽略大小写，同时它还会对特殊字符进行转换
	strEqual := strings.EqualFold("AB大", "ab大")
	fmt.Println("对比字符串是否相等=", strEqual)
	// 结果：true

	fmt.Println("======================================")

	// 以换行分隔字符串
	lines := bufio.NewScanner(strings.NewReader("以\n换\r\n分隔\n字符串"))
	lines.Split(bufio.ScanLines)
	for lines.Scan() {
		fmt.Println(lines.Text())
	}

	// 替换字符串(多个)-使用特定规则(成对出现)对字符串进行替换操作
	var replacer = strings.NewReplacer(
		"&", "&amp;", // 把 & 替换为 &amp
		"<", "&lt;",
		">", "&gt;",
		`"`, "&#34;",
		"'", "&#39;",
	)
	fmt.Println(replacer.Replace("替换<字符串>-使用\"特定\"规则对字符串进行'替换'操作"))
}

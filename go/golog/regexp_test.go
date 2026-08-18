package main

import (
	"fmt"
	"regexp"
	"testing"
)

/*
正则表达式：regexp 包（对应 TS regex.test.ts / Python re 模块）

Go 正则语法：RE2（不支持回溯引用 \1、环视 lookahead，与 Perl/Python 有差异）
*/

// 判断是否匹配（对应 Python re.match / JS test）
func TestRegexMatch(t *testing.T) {
	// 简单匹配：手机号（1 开头，11 位数字）
	phone := "13800138000"
	matched, _ := regexp.MatchString(`^1[3-9]\d{9}$`, phone)
	fmt.Println("手机号匹配:", matched) // true

	// 邮箱
	matched2, _ := regexp.MatchString(`^\w+@\w+\.\w+$`, "laixhe@example.com")
	fmt.Println("邮箱匹配:", matched2) // true

	// 匹配不存在的
	fmt.Println("非手机号:", regexp.MustCompile(`^1[3-9]\d{9}$`).MatchString("12345"))
}

// 查找（对应 Python re.search / re.findall / JS match）
func TestRegexFind(t *testing.T) {
	re := regexp.MustCompile(`\d+`) // 连续数字

	// 查找第一个
	fmt.Println("第一个:", re.FindString("价格 100 元，优惠 20 元")) // 100

	// 查找所有
	fmt.Println("所有:", re.FindAllString("价格 100 元，优惠 20 元", -1)) // [100 20]

	// 查找所有（限制数量）
	fmt.Println("前 1 个:", re.FindAllString("a1 b22 c333", 1)) // [1]

	// 查找下标（返回匹配位置区间）
	fmt.Println("位置:", re.FindStringIndex("a1 b22")) // [1 2]
}

// 捕获分组（对应 Python 分组 / JS 捕获组）
func TestRegexGroup(t *testing.T) {
	re := regexp.MustCompile(`(\d{4})-(\d{2})-(\d{2})`) // 年-月-日
	match := re.FindStringSubmatch("日期：2026-03-04")
	fmt.Println("完整:", match[0]) // 2026-03-04
	fmt.Println("年:", match[1], "月:", match[2], "日:", match[3])
}

// 替换（对应 Python re.sub / JS replace）
func TestRegexReplace(t *testing.T) {
	// 替换所有匹配
	re := regexp.MustCompile(`\d+`)
	fmt.Println("替换:", re.ReplaceAllString("年龄 18 岁，得分 88", "[数字]"))
	// [数字] 岁，得分 [数字]

	// 模板替换：用 $1 引用分组
	re2 := regexp.MustCompile(`(\d{4})-(\d{2})-(\d{2})`)
	fmt.Println("模板:", re2.ReplaceAllString("2026-03-04", "$2/$3/$1")) // 03/04/2026

	// 删除（替换为空串）
	fmt.Println("删除数字:", re.ReplaceAllString("abc123def", "")) // abcdef
}

// 预编译复用（对应 Python 编译 re.compile / JS 创建 RegExp 对象）
func TestRegexCompile(t *testing.T) {
	// 方法1：MustCompile（编译失败直接 panic，适合程序启动时用常量）
	re := regexp.MustCompile(`^[a-z]+$`)

	// 方法2：Compile（返回 error，适合用户输入的正则）
	re2, err := regexp.Compile(`^[a-z]+$`)
	fmt.Println("Compile:", err) // <nil>

	// 编译后的正则复用：多行匹配
	text := "abc\ndef\n"
	fmt.Println("多行匹配:", re.MatchString(text))   // false（^ 只匹配整个文本开头）
	fmt.Println("多行模式:", re2.MatchString("abc")) // true
}

// 常用正则场景汇总（手机号 / 邮箱 / IP 地址）
func TestRegexCommon(t *testing.T) {
	cases := map[string]struct {
		pattern string
		input   string
	}{
		"手机号":  {"^1[3-9]\\d{9}$", "13800138000"},
		"邮箱":   {"^\\w+@\\w+\\.\\w+$", "laixhe@example.com"},
		"IPv4": {"^(\\d{1,3}\\.){3}\\d{1,3}$", "192.168.1.1"},
	}
	for name, c := range cases {
		fmt.Println(name, regexp.MustCompile(c.pattern).MatchString(c.input))
	}
}

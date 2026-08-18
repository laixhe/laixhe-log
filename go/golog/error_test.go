package main

import (
	"errors"
	"fmt"
	"testing"
)

/*
错误处理：error 接口 / panic / recover（对应 Python exceptions.py / Java 异常机制）

Go 与 Python/Java 的区别：
- 错误是普通返回值（error 接口），不是异常，调用方必须显式处理（区别于 try/except）
- panic ≈ 运行时异常，recover ≈ 捕获异常（对应 Python except）
*/

// 自定义错误：实现 error 接口（Error() string 方法）
type DivideError struct {
	A int
	B int
}

func (e DivideError) Error() string {
	return fmt.Sprintf("无法计算 %d / %d：除数不能为 0", e.A, e.B)
}

// 返回错误（对应 Python raise / Java throw，但 Go 用返回值）
func safeDivide(a, b int) (int, error) {
	if b == 0 {
		return 0, DivideError{A: a, B: b}
	}
	return a / b, nil
}

// 包级错误变量（惯例：ErrXxx 命名），供 errors.Is 比较
var ErrMissingEqual = errors.New("缺少 '=' 分隔符")

// 错误包装（Go 1.13+ %w，保留错误链，对应 Python raise from）
func parseConfig(conf string) error {
	return fmt.Errorf("配置文件无效: %w", ErrMissingEqual)
}

// panic + recover（对应 Python try/except）
func riskyDiv(a, b int) (result int) {
	// defer + recover：恢复 panic，把程序从崩溃中救回
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("捕获 panic:", r)
			result = -1 // 设置默认返回值
		}
	}()
	if b == 0 {
		panic(fmt.Sprintf("除以 0 了：%d/%d", a, b)) // 对应 Python raise
	}
	return a / b
}

func TestError(t *testing.T) {
	// 错误作为返回值：必须处理（区别于 Python 不处理也不会崩）
	result, err := safeDivide(10, 2)
	if err != nil {
		fmt.Println("出错:", err)
	} else {
		fmt.Println("10/2 =", result)
	}

	_, err = safeDivide(10, 0)
	// errors.As：匹配自定义错误类型（对应 Python except 特定类型）
	var de DivideError
	if errors.As(err, &de) {
		fmt.Println("捕获 DivideError:", de)
	}
	fmt.Println("错误字符串:", err) // 自动调用 Error() 方法
}

func TestErrorWrap(t *testing.T) {
	err := parseConfig("a=1")
	fmt.Println("包装错误:", err)
	// errors.Is：判断错误链中是否包含某个已知错误（对应 Python 检查异常原因）
	fmt.Println("包含底层错误?", errors.Is(err, ErrMissingEqual)) // true
}

func TestPanicRecover(t *testing.T) {
	// panic 默认导致程序崩溃，recover 在 defer 中捕获
	fmt.Println("panic/recover 演示:")
	r := riskyDiv(10, 0)
	fmt.Println("返回值:", r) // -1（recover 后设置的默认值）
	fmt.Println("程序继续运行（未崩溃）")

	// 正常路径
	fmt.Println("10/2 =", riskyDiv(10, 2))
}

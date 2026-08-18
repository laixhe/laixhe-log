# Go 学习笔记

个人 Go 学习笔记整理：标准库速查 + 命令参考 + 专题深入 + 一个示例项目。

## 目录

### 语法速查与专题笔记

| 文件 | 主题 |
|------|------|
| `go-命令.md` | go build / clean / tool 命令、交叉编译、调试工具清单 |
| `go-work-mod管理.md` | go.work 工作区 / go.mod 模块、require / replace |
| `go-构建标识.md` | 构建约束 build tags（`// +build`） |
| `go-内存逃逸.md` | 内存逃逸分析（栈 vs 堆） |
| `cmd.md` | 命令行参数（os.Args / flag / Scanf / bufio） |
| `regexp.md` | 正则表达式（匹配 / 替换 / 常用校验） |
| `go-docker.md` | Docker 打包 Go（多阶段构建） |
| `go-其它.md` | Makefile、unsafe.Sizeof、位运算 flags |

### 项目

| 项目 | 说明 |
|------|------|
| [golog](./golog/README.md) | Go 标准库 + 常用场景示例（slices / strings / time / sync / http / json 等） |

---

### 标准库
```
fmt       格式化输入输出
io        通用 I/O 接口（Reader, Writer）
os        操作系统交互（文件、目录、进程）
time      时间与日期处理
sync      并发同步原语（WaitGroup协程同步, Mutex RWMutex互斥锁, Once单例模式）
strings   字符串处理
strconv   类型转换
math      数学运算与随机数
regexp    正则表达式
sort      排序（切片 Ints, Strings）
flag      命令行参数解析
crypto    加密
hash      哈希
context   上下文管理
container 容器数据结构（heap堆实现, list双向链表, ring环形链表）
```

#### 默认值
```
没有初始化的基本类型默认为零值 (数值为 0 | 字符串为 "" | 布尔为 false)
可根据值自行判定变量类型
```

#### 第一个例子
```go
package main

import (
	"fmt"
)

// GetAdd 定义一个相加的函数
func GetAdd(num1, num2 int) int {
	return num1 + num2
}

func main() {
	// 声明变量并初始化
	var hello string = "Hello"
	// 定义变量并根据值自动推断类型（简短声明方式（只能在函数内部使用））
	world := "World"
	s1 := fmt.Sprintf("%s %s", hello, world)
	s2 := hello + " " + world
	fmt.Println(s1) // 结果: Hello World
	fmt.Println(s2) // 结果: Hello World

	// 定义变量并调用相加函数
	numberInt := GetAdd(1, 2)
	fmt.Println(numberInt) // 结果: 3
}
```

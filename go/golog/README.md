# golog

Go 语言学习项目：用测试文件（`*_test.go`）承载各种标准库与常用场景示例，代码含中文注释，可逐个运行观察输出。

## 环境要求

| 项目 | 要求 |
|------|------|
| **Go 版本** | 1.24+（`go.mod` 声明 `go 1.26`；`json_test.go` 用到的 `omitzero` 是 Go 1.24 新特性） |
| 查看版本 | `go version` |

## 项目结构

| 文件 | 主题 | 说明 |
|------|------|------|
| `basic_test.go` | 基础类型 | 基本数据类型 / 变量声明 / 常量与 iota / 指针 |
| `array_test.go` | 数组 | 声明 / 值语义（赋值传参拷贝）/ 指针传参 / 多维数组 / range 遍历 |
| `control_flow_test.go` | 控制流 | if / for（含 while、无限循环、标号）/ range / switch（fallthrough） |
| `function_test.go` | 函数 | 多返回值 / 命名返回值 / 可变参数 / 闭包 / defer |
| `generic_test.go` | 泛型 | 泛型函数 / 泛型结构体 / 类型约束（constraints）/ `~` 底层类型 |
| `type_test.go` | 结构体与接口 | 结构体 / 值与指针接收者 / 接口（隐式实现）/ 类型断言 / 类型 switch |
| `embed_test.go` | 结构体嵌入 | 匿名字段嵌入 / 字段与方法提升 / 方法覆盖 / 多层嵌入 / 嵌入接口 |
| `error_test.go` | 错误处理 | error 接口 / errors.Is&As / %w 包装 / panic+recover |
| `regexp_test.go` | 正则 | `regexp` 包：匹配 / 查找 / 捕获分组 / 替换 / 预编译复用 |
| `fileio_test.go` | 文件读写 | os.WriteFile/ReadFile / bufio 逐行 / 追加 / 目录与重命名删除 / filepath |
| `goroutine_test.go` | goroutine/channel | goroutine / 无缓冲与缓冲 channel / close+range / select / 只读只写 / 并发求和 |
| `slice_test.go` | 切片 | 基础（声明/make/len/cap/append 扩容/copy/引用语义）+ 进阶（`slices` 包：Contains/Sort/Reverse/Compact/SortFunc） |
| `string_test.go` | 字符串 | `strings` 常用方法、byte/rune/len 的区别 |
| `map_test.go` | 字典 | 基础（声明/增删改查/遍历/nil map/引用共享/嵌套）+ 进阶（`maps` + `slices` 迭代器：Keys/Sorted/Collect） |
| `time_test.go` | 时间 | Now / Format / Parse / 时区 / 定时器 / 耗时 |
| `sync_test.go` | 并发同步 | Once / WaitGroup / Mutex / atomic |
| `container_list_test.go` | 双向链表 | `container/list` |
| `container_ring_test.go` | 环形链表 | `container/ring` |
| `container_heap_test.go` | 堆 | `container/heap` |
| `lru_test.go` | LRU 缓存 | 手写 LRU（`container/list` + `map`） |
| `json_test.go` | JSON | `encoding/json` Tag：omitempty / omitzero / string |
| `http_serve_test.go` | HTTP 服务端 | `net/http` + HTTP/3（quic-go） |
| `http_client_test.go` | HTTP 客户端 | `net/http` GET/POST + HTTP/3 |
| `unique_test.go` | 去重 | 占位示例 |
| `number_test.go` | 数值进阶 | 格式化 / 溢出与饱和 / 类型转换（`strconv`） |
| `iterators_test.go` | 迭代器 | Go 1.23+ `range over func`：自定义 `iter.Seq` / map / filter / take / skip / zip / 综合实战 |

## 运行

所有示例都通过 `go test` 承载，建议按需运行单个测试：

```bash
# 运行单个测试（推荐）
go test -run TestBasicTypes -v
go test -run TestArrayDeclare -v
go test -run TestFor -v
go test -run TestFunction -v
go test -run TestGenericFunction -v
go test -run TestStruct -v
go test -run TestEmbed -v
go test -run TestError -v
go test -run TestRegexMatch -v
go test -run TestFileWriteRead -v
go test -run TestGoroutine -v
go test -run TestSliceDeclare -v
go test -run TestMapCRUD -v
go test -run TestSliceHandle -v
go test -run TestString -v
go test -run TestLRU -v
go test -run TestJson -v
go test -run TestNumber -v
go test -run TestIter -v

# 运行全部
go test -v
```

> ⚠️ 说明：部分示例会持续运行/阻塞，属于演示性质，请按需运行单个 `-run`：
> - `http_serve_test.go` 的 `ListenAndServe` 会一直监听端口
> - `time_test.go` 的定时器类会 sleep 较长时间
> - `http_client_test.go` 需要访问外部网络

## 学习路径建议（从易到难）

1. **基础语法**：`basic_test.go` → `array_test.go` → `control_flow_test.go` → `function_test.go` → `generic_test.go` → `type_test.go` → `embed_test.go` → `error_test.go`
2. **正则**：`regexp_test.go`
3. **文件读写**：`fileio_test.go`
4. **并发基础**：`goroutine_test.go`（channel / select）→ `sync_test.go`（锁与原子）
5. **基础容器**：`slice_test.go`（基础部分）→ `map_test.go`（基础部分）→ `string_test.go`
6. **容器进阶**：`slice_test.go`（slices 包）→ `map_test.go`（maps 包）
7. **容器结构**：`container_list_test.go` → `container_ring_test.go` → `container_heap_test.go` → `lru_test.go`
8. **数值进阶**：`number_test.go`（格式化 / 溢出 / 类型转换）
9. **迭代器**：`iterators_test.go`（Go 1.23+ `range over func`）
10. **时间与并发**：`time_test.go` → `sync_test.go`
11. **序列化**：`json_test.go`
12. **网络**：`http_client_test.go` → `http_serve_test.go`

> 跨语言对照学习建议：同一主题在 Rust（`rust/rustlog`）、Java（`java/javalog`）、C#（`csharp/cslog`）等项目中并列阅读，观察语言各自的惯用写法（如 Go 迭代器 vs Rust 迭代器 vs C# LINQ；Go 饱和运算 vs Rust saturating_add 等）。

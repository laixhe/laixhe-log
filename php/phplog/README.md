# phplog

PHP 学习项目：参考 Go [golog](../../go/golog/README.md) 与 Rust [rustlog](../../rust/rustlog/README.md)，使用 **PHP 8.5 + Composer + PHPUnit**，以 Demo 类（`php run.php`）与 PHPUnit 测试类（`tests/*Test.php`）承载各种标准库与常用库示例，代码含中文注释，可逐个运行观察输出。

## 环境要求

| 项目 | 要求 |
|------|------|
| **PHP 版本** | 8.1+（示例使用 Fiber，推荐 8.5，`php -v` 查看） |
| **扩展** | mbstring、curl、json（内置）、sockets（`http_server.php` 使用） |
| **Composer** | 2.x（安装 PHPUnit） |

> ⚠️ Windows 控制台如中文乱码，先执行 `chcp 65001` 再运行。

## 项目结构

### 主程序（`php run.php`，对应 Rust `cargo run`）

| Demo 类 | 主题 | 对应参考 | 说明 |
|------|------|------|------|
| [NumberDemo.php](src/NumberDemo.php) | 数值类型 | Rust `number.rs` | 格式化输出、整数溢出（提升 float / 除零异常）、类型转换 |
| [CharStringDemo.php](src/CharStringDemo.php) | 字符与字符串 | Rust `char_string.rs` / Go `string_test.go` | strlen vs mb_strlen、str_contains 系列、词频统计、解析 |
| [CollectionDemo.php](src/CollectionDemo.php) | 集合类型 | Rust `array_map.rs` / Go `container/*`、`lru_test.go` | 数组（列表/字典）、SplDeque/SplQueue/SplStack/SplPriorityQueue、切片/去重/分组 |
| [LRUCache.php](src/LRUCache.php) | LRU 缓存 | Go `lru_test.go` | SplDoublyLinkedList + 关联数组实现 LRU |
| [RingBuffer.php](src/RingBuffer.php) | 环形缓冲区 | Go `container/ring` | 固定容量环，写满覆盖最旧数据 |
| [StreamDemo.php](src/StreamDemo.php) | ⭐ 流/数组函数 | Rust `iterators.rs` | array_map/filter/reduce、生成器（yield）、zip/flatten/partition、综合实战 |
| [TimeDemo.php](src/TimeDemo.php) | 时间处理 | Go `time_test.go` | DateTime/时间戳、格式化解析、时区、耗时、定时器 |
| [SyncDemo.php](src/SyncDemo.php) | 并发同步 | Go `sync_test.go` | Fiber 协程、只执行一次、文件锁、PHP 单线程模型说明 |
| [JsonDemo.php](src/JsonDemo.php) | JSON 序列化 | Go `json_test.go` | json_encode/decode、omitempty 手动实现、错误处理 |
| [HttpDemo.php](src/HttpDemo.php) | HTTP 客户端 | Go `http_client_test.go` | cURL GET/POST/JSON、proc_open 启动服务器辅助 |

入口 [run.php](run.php) 串联全部 Demo，对应 Rust `main.rs`；[http_server.php](http_server.php) 为独立 HTTP 服务器，对应 Go `http_serve_test.go`。

### 测试（`composer test`，对应 Go `go test -v` / Rust `cargo test`）

| 测试类 | 对应参考 | 核心断言 |
|------|------|------|
| [NumberTest.php](tests/NumberTest.php) | Rust `number.rs` 练习题 | 进制补零、溢出提升 float、filter_var 校验 |
| [CharStringTest.php](tests/CharStringTest.php) | Rust `char_string.rs` 练习题 | trim/split、汉字区间、字节/字符数 |
| [CollectionTest.php](tests/CollectionTest.php) | Rust `array_map.rs` / Go lru、slice | 有序 key、SplPriorityQueue、LRU 淘汰、环形覆盖 |
| [StreamTest.php](tests/StreamTest.php) | Rust `iterators.rs` 练习题 | 偶数平方和、zip/flatten、平均月薪 52500 |
| [TimeTest.php](tests/TimeTest.php) | Go `time_test.go` | 格式化解析、时区差、比较、耗时 |
| [SyncTest.php](tests/SyncTest.php) | Go `sync_test.go` | 只执行一次、Fiber 调度顺序 |
| [JsonTest.php](tests/JsonTest.php) | Go `json_test.go` | omitempty、字符串数字、错误处理 |
| [HttpTest.php](tests/HttpTest.php) | Go `http_*_test.go` | proc_open 启动服务器后 GET/POST 断言 |

另有 [yaf/](./yaf/) 目录：Yaf 框架示例项目（MVC + 多模块 + 数据库 + 日志），详见 [yaf/README.md](./yaf/README.md)。

## 运行

```bash
# 首次安装依赖（PHPUnit）
composer install

# 运行全部测试（推荐，类似 go test -v）
composer test        # 等价 vendor\bin\phpunit

# 运行单个测试类
vendor\bin\phpunit tests\StreamTest.php

# 运行主程序，串联输出全部 Demo（类似 cargo run）
php run.php
```

HTTP 示例的两种运行方式：

```bash
# 方式一：自动（推荐）—— run.php 内部用 proc_open 启动服务器子进程
php run.php

# 方式二：手动两终端（对应 Rust TCP 示例 / Go http_serve）
# 终端 1：
php http_server.php        # 输出 127.0.0.1:PORT（默认随机端口）
# 终端 2：
php -r "require 'vendor/autoload.php'; echo Laixhe\Phplog\HttpDemo::clientGet('http://127.0.0.1:PORT/get?name=laixhe');"
```

## 学习路径建议（从易到难）

1. **基础类型**：[NumberDemo.php](src/NumberDemo.php) → [CharStringDemo.php](src/CharStringDemo.php)
2. **集合类型**：[CollectionDemo.php](src/CollectionDemo.php)（数组 → SPL 容器 → LRU / 环形缓冲）
3. **迭代器/流**：[StreamDemo.php](src/StreamDemo.php)
4. **时间与并发**：[TimeDemo.php](src/TimeDemo.php) → [SyncDemo.php](src/SyncDemo.php)
5. **序列化**：[JsonDemo.php](src/JsonDemo.php)
6. **网络**：[HttpDemo.php](src/HttpDemo.php) + [http_server.php](http_server.php)

> 跨语言对照学习建议：同一主题在 Go（`go/golog`）、Rust（`rust/rustlog`）、Java（`java/javalog`）、PHP（`php/phplog`）四份代码中并列阅读，观察语言各自的惯用写法（如 Go 切片 vs Rust Vec vs Java ArrayList vs PHP 数组、Go goroutine vs Java 线程池 vs PHP Fiber 等）。

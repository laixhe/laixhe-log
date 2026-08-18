# javalog

Java 语言学习项目：参考 Go [golog](../../go/golog/README.md) 与 Rust [rustlog](../../rust/rustlog/README.md)，使用 **GraalVM 25 + Gradle Kotlin DSL 9.7**，以 JUnit 测试类（`*Test.java`）与 Demo 类（`gradlew run`）承载各种标准库与常用库示例，代码含中文注释，可逐个运行观察输出。

## 环境要求

| 项目 | 要求 |
|------|------|
| **Java 版本** | GraalVM 25 / JDK 25+（`java --version` 查看） |
| **Gradle** | 9.7（使用项目自带 wrapper，无需全局安装） |

> 项目通过 [build.gradle.kts](build.gradle.kts) 声明 `JavaLanguageVersion.of(25)` 工具链，首次构建会自动探测本机 JDK 25。

## 项目结构

### 主程序（`gradlew run`，对应 Rust `cargo run`）

| Demo 类 | 主题 | 对应参考 | 说明 |
|------|------|------|------|
| [NumberDemo.java](src/main/java/com/laixhe/javalog/demo/NumberDemo.java) | 数值类型 | Rust `number.rs` | 格式化输出、整数溢出（addExact / clamp / 回绕）、类型转换 |
| [CharStringDemo.java](src/main/java/com/laixhe/javalog/demo/CharStringDemo.java) | 字符与字符串 | Rust `char_string.rs` / Go `string_test.go` | char 码点、UTF-16/码点/字节长度、String 常用方法、词频统计 |
| [CollectionDemo.java](src/main/java/com/laixhe/javalog/demo/CollectionDemo.java) | 集合类型 | Rust `array_map.rs` / Go `container/*`、`lru_test.go` 等 | ArrayList / ArrayDeque / HashMap / TreeMap / HashSet / TreeSet / PriorityQueue / LinkedList、环形缓冲区、手写 LRU、切片/字典/去重/分组 |
| [StreamDemo.java](src/main/java/com/laixhe/javalog/demo/StreamDemo.java) | ⭐ 流/迭代器 | Rust `iterators.rs` | map / filter / flatMap / limit / skip、reduce / partition、综合实战 |
| [TimeDemo.java](src/main/java/com/laixhe/javalog/demo/TimeDemo.java) | 时间处理 | Go `time_test.go` | Instant / LocalDateTime、格式化解析、时区、耗时、定时器 |
| [SyncDemo.java](src/main/java/com/laixhe/javalog/demo/SyncDemo.java) | 并发同步 | Go `sync_test.go` | Once / CountDownLatch / ReentrantLock / 读写锁 / AtomicInteger |
| [JsonDemo.java](src/main/java/com/laixhe/javalog/demo/JsonDemo.java) | JSON 序列化 | Go `json_test.go` | Jackson 注解：omitempty / omitzero 对应、数字转字符串、@JsonIgnore |
| [HttpDemo.java](src/main/java/com/laixhe/javalog/demo/HttpDemo.java) | HTTP 服务端与客户端 | Go `http_serve_test.go` / `http_client_test.go` | JDK 内置 HttpServer + HttpClient（GET / POST / JSON） |

入口 [Main.java](src/main/java/com/laixhe/javalog/Main.java) 串联全部 Demo，对应 Rust `main.rs`。

### 测试（`gradlew test`，对应 Go `go test -v` / Rust `cargo test`）

| 测试类 | 对应参考 | 核心断言 |
|------|------|------|
| [NumberTest.java](src/test/java/com/laixhe/javalog/NumberTest.java) | Rust `number.rs` 练习题 | 进制补零、溢出检测、Math.addExact |
| [CharStringTest.java](src/test/java/com/laixhe/javalog/CharStringTest.java) | Rust `char_string.rs` 练习题 | trim/split、汉字区间、长度字节/码点 |
| [CollectionTest.java](src/test/java/com/laixhe/javalog/CollectionTest.java) | Rust `array_map.rs` / Go lru、slice | TreeMap 有序、LRU 淘汰、环形覆盖、排序 |
| [StreamTest.java](src/test/java/com/laixhe/javalog/StreamTest.java) | Rust `iterators.rs` 练习题 | 偶数平方和、flatMap 展开、平均月薪 52500 |
| [TimeTest.java](src/test/java/com/laixhe/javalog/TimeTest.java) | Go `time_test.go` | 格式化解析、时区差、比较、耗时 |
| [SyncTest.java](src/test/java/com/laixhe/javalog/SyncTest.java) | Go `sync_test.go` | Once、等待线程、计数 1000 |
| [JsonTest.java](src/test/java/com/laixhe/javalog/JsonTest.java) | Go `json_test.go` | omitempty、字符串数字、反序列化 |
| [HttpTest.java](src/test/java/com/laixhe/javalog/HttpTest.java) | Go `http_*_test.go` | 服务端启动后 GET / POST 断言 |

## 运行

所有示例通过 Gradle 承载，建议按需运行：

```bash
# 运行全部测试（推荐，类似 go test -v）
gradlew.bat test

# 运行单个测试类
gradlew.bat test --tests "com.laixhe.javalog.StreamTest"

# 运行单个测试方法
gradlew.bat test --tests "com.laixhe.javalog.JsonTest.exercise4_deserialize"

# 运行主程序，串联输出全部 Demo（类似 cargo run）
gradlew.bat run
```

> ⚠️ 说明：
> - `gradlew.bat test` 会输出所有 Demo 的打印结果（已配置 `showStandardStreams = true`）
> - [TimeDemo.java](src/main/java/com/laixhe/javalog/demo/TimeDemo.java) 的 `timeTicker` 定时器会阻塞约 3 秒，`runTimeTicker` 测试按需单独运行
> - HTTP 示例启动的是本机随机端口服务，无外部网络依赖

## 学习路径建议（从易到难）

1. **基础类型**：[NumberDemo.java](src/main/java/com/laixhe/javalog/demo/NumberDemo.java) → [CharStringDemo.java](src/main/java/com/laixhe/javalog/demo/CharStringDemo.java)
2. **集合类型**：[CollectionDemo.java](src/main/java/com/laixhe/javalog/demo/CollectionDemo.java)（ArrayList → ArrayDeque → Map/Set → PriorityQueue → LinkedList → LRU）
3. **迭代器/流**：[StreamDemo.java](src/main/java/com/laixhe/javalog/demo/StreamDemo.java)
4. **时间与并发**：[TimeDemo.java](src/main/java/com/laixhe/javalog/demo/TimeDemo.java) → [SyncDemo.java](src/main/java/com/laixhe/javalog/demo/SyncDemo.java)
5. **序列化**：[JsonDemo.java](src/main/java/com/laixhe/javalog/demo/JsonDemo.java)
6. **网络**：[HttpDemo.java](src/main/java/com/laixhe/javalog/demo/HttpDemo.java)

> 跨语言对照学习建议：同一主题在 Go（`go/golog`）、Rust（`rust/rustlog`）、Java（`java/javalog`）三份代码中并列阅读，观察语言各自的惯用写法（如 Go 切片 vs Rust Vec vs Java ArrayList、Go goroutine vs Java 线程池等）。

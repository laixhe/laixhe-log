# pylog

Python 学习示例项目（用 uv 管理），参考 Go [golog](../../go/golog/README.md) 与 Rust [rustlog](../../rust/rustlog/README.md)，按主题拆分模块，逐个演示基础语法与进阶主题，代码含中文注释，可 `uv run` 直接运行。

## 环境要求

| 项目 | 要求 |
|------|------|
| **Python** | 3.14+（`.python-version` 已锁定 3.14） |
| **uv** | 用于项目管理与运行（`uv --version` 查看） |

## 项目结构

### 基础主题（按学习顺序）

| 模块 | 主题 |
|------|------|
| `basic_types.py` | 变量与基本数据类型（int / float / bool / str / None + 类型转换） |
| `control_flow.py` | 控制流（if / for / while） |
| `functions.py` | 函数（定义 / 默认参数 / `*args` / `**kwargs` / lambda） |
| `collections.py` | 集合（list / tuple / dict / set + 列表推导式） |
| `strings.py` | 字符串（f-string / 常用方法 / 切片） |
| `classes.py` | 类与对象（构造方法 / 实例方法 / 继承） |
| `exceptions.py` | 异常处理（try / except / finally / 自定义异常） |
| `date_time.py` | 时间与日期（datetime / time：格式化 / 时间戳 / 解析 / 比较） |
| `regex.py` | 正则（re：手机号 / 邮箱大小写不敏感） |

### 进阶主题（对应 Go golog / Rust rustlog）

| 模块 | 主题 | 对应参考 | 说明 |
|------|------|------|------|
| `number.py` | 数值类型进阶 | Rust `number.rs` | 格式化（hex/八进制/对齐/千分位）、Python int 无溢出、类型转换 |
| `iterators.py` | ⭐ 迭代器与推导式 | Rust `iterators.rs` | 推导式 / map / filter / zip / itertools / 生成器 / 综合实战 |
| `containers.py` | 容器进阶 | Go `container/*`、`lru_test.go`、slice/map/unique | deque / heapq / LRU / 环形缓冲 / 去重 / 分组 / 排序 |
| `concurrency.py` | 并发同步 | Go `sync_test.go` | threading：Once / join / Lock / 线程池（GIL 说明） |
| `json_demo.py` | JSON 序列化 | Go `json_test.go` | json.dumps / loads、omitempty 手动实现、美化输出、错误处理 |
| `http_demo.py` | HTTP 服务与客户端 | Go `http_serve_test.go` / `http_client_test.go` | http.server + urllib：GET / POST 表单 / JSON，后台线程自包含 |

## 运行

```bash
# 首次同步依赖（pytest / ruff）
uv sync

# 运行全部主题（按学习顺序，对应 cargo run）
uv run python -m pylog

# 运行单个主题模块
uv run python -c "from pylog import iterators; iterators.run()"

# 运行全部测试（对应 go test -v / cargo test）
uv run pytest -q

# 代码检查（ruff）
uv run ruff check .
```

## 学习路径

按 `__init__.py` 中 `main()` 的调用顺序递进：

1. **基本数据类型** → `basic_types.py`
2. **控制流** → `control_flow.py`
3. **函数** → `functions.py`
4. **集合** → `collections.py`
5. **字符串** → `strings.py`
6. **类与对象** → `classes.py`
7. **异常处理** → `exceptions.py`
8. **时间与日期** → `date_time.py`
9. **正则** → `regex.py`
10. **数值类型进阶** → `number.py`
11. **迭代器与推导式** → `iterators.py`
12. **容器进阶** → `containers.py`
13. **并发同步** → `concurrency.py`
14. **JSON 序列化** → `json_demo.py`
15. **HTTP 服务与客户端** → `http_demo.py`

> 跨语言对照学习建议：同一主题在 Go（`go/golog`）、Rust（`rust/rustlog`）、Java（`java/javalog`）、PHP（`php/phplog`）、Python（`python/pylog`）五份代码中并列阅读，观察语言各自的惯用写法（如 Go 切片 vs Rust Vec vs Java ArrayList vs PHP 数组 vs Python list、Go goroutine vs Java 线程池 vs PHP Fiber vs Python threading 等）。

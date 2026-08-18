# tslog

TypeScript 语言学习项目：用 `bun test` 承载各主题示例，代码含中文注释，可逐个运行观察输出。

## 环境要求

| 项目 | 要求 |
|------|------|
| **Bun** | 1.x（集成运行时 + 包管理器 + 测试运行器） |
| 查看版本 | `bun --version` |

Bun 内置 TypeScript 转译，无需单独安装 `tsc` 即可直接运行 `.ts` 文件。

## 项目结构

| 文件 | 主题 | 说明 |
|------|------|------|
| `src/basics.test.ts` | 基础类型 | 原始类型 / 数组 / 元组 / 枚举 / 联合 / 字面量 |
| `src/functions.test.ts` | 函数 | 参数与返回类型 / 可选参数 / 默认值 / 重载 |
| `src/interfaces.test.ts` | 接口与类型别名 | interface / type / 交叉类型 / 扩展 |
| `src/generics.test.ts` | 泛型 | 泛型函数 / 泛型接口 / 泛型约束 |
| `src/classes.test.ts` | 类 | 继承 / 访问修饰符 / getter/setter / 抽象类 |
| `src/async.test.ts` | 异步 | Promise / async/await / 错误处理 |
| `src/time.test.ts` | 时间与日期 | 时间戳 / 格式化 / 字符串解析 / 时间比较（date-fns） |
| `src/regex.test.ts` | 正则 | 手机号 / 邮箱匹配（RegExp） |
| `src/utility-types.test.ts` | 工具类型 | Partial / Required / Pick / Omit / Record 等 |
| `src/number.test.ts` | 数值进阶 | 格式化 / 精度与溢出（BigInt）/ 类型转换 |
| `src/strings.test.ts` | 字符串 | 码元 / 码点 / 字节长度、常用方法、词频统计 |
| `src/collections.test.ts` | 容器进阶 | 数组 / Map / Set / 最小堆 / 环形缓冲 / LRU / 去重 / 分组 |
| `src/iterators.test.ts` | ⭐ 迭代器与数组方法 | map / filter / reduce / zip / flatMap / partition / 综合实战 |
| `src/json.test.ts` | JSON | stringify / parse / omitempty / 美化输出 / replacer |
| `src/http.test.ts` | HTTP | Bun.serve 服务端 + fetch 客户端（GET / POST / JSON） |

## 运行

```bash
# 安装依赖（bun-types 提供 bun:test 类型提示，typescript 用于类型检查）
bun install

# 运行所有测试
bun test

# 只运行某个主题
bun test src/basics.test.ts

# 类型检查（bun 转译不做类型检查，严格模式需单独跑 tsc）
bun run typecheck
```

> 说明：Bun 运行时只做语法转译、不做类型检查，因此类型错误不会导致 `bun test` 失败；类型是否正确由 `bun run typecheck`（`tsc --noEmit`）保证。

## 学习路径建议（从易到难）

1. **基础类型**：`src/basics.test.ts`
2. **函数**：`src/functions.test.ts`
3. **接口与类型别名**：`src/interfaces.test.ts`
4. **泛型**：`src/generics.test.ts`
5. **类**：`src/classes.test.ts`
6. **异步**：`src/async.test.ts`
7. **数值进阶**：`src/number.test.ts`
8. **字符串**：`src/strings.test.ts`
9. **迭代器与数组方法**：`src/iterators.test.ts`
10. **容器进阶**：`src/collections.test.ts`
11. **时间与日期**：`src/time.test.ts`
12. **正则**：`src/regex.test.ts`
13. **工具类型**：`src/utility-types.test.ts`
14. **JSON**：`src/json.test.ts`
15. **HTTP**：`src/http.test.ts`

> 跨语言对照学习建议：同一主题在 Go（`go/golog`）、Rust（`rust/rustlog`）、Java（`java/javalog`）、PHP（`php/phplog`）、Python（`python/pylog`）、Zig（`zig/ziglog`）、TypeScript（`web/tslog`）七份代码中并列阅读，观察语言各自的惯用写法（如 Go 切片 vs Rust Vec vs Java ArrayList vs PHP 数组 vs Python list vs TS Array、Go goroutine vs Java 线程池 vs PHP Fiber vs Python threading vs TS Promise 等）。

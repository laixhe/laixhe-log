import { test, expect } from "bun:test";

// 字符串操作：长度 / 编码 / 常用方法 / 词频统计。
// 对应 Go golog/string_test.go 与 Rust rustlog/src/char_string.rs。
//
// 前置知识：
// - s.length 是 UTF-16 码元数（一个中文 1 个，Emoji 2 个）→ 对应 Go len() 的「字符数」误区
// - [...s].length 是 Unicode 码点数（对应 Rust chars().count() / Go 的 rune 数）
// - new TextEncoder().encode(s).length 是 UTF-8 字节数（对应 Go len([]byte(s))）

test("字符串长度：码元 / 码点 / 字节", () => {
  const s = "你好😀";
  expect(s.length).toBe(4); // UTF-16 码元数（😀 占 2 个）
  expect([...s].length).toBe(3); // Unicode 码点数
  expect(new TextEncoder().encode(s).length).toBe(10); // UTF-8 字节数（3+3+4）
});

test("字符串常用方法", () => {
  const s = "   Hello, Rust! I love Rust.   ";

  // 查找（对应 strings.Contains / Index / LastIndex）
  expect(s.includes("Rust")).toBe(true);
  expect(s.startsWith("   He")).toBe(true);
  expect(s.endsWith(".   ")).toBe(true);
  expect(s.indexOf("Rust")).toBe(10);
  expect(s.lastIndexOf("Rust")).toBe(23);
  expect("abc".indexOf("z")).toBe(-1);

  // 去空白 / 替换（对应 strings.Trim / ReplaceAll）
  expect(s.trim()).toBe("Hello, Rust! I love Rust.");
  expect(s.replaceAll("Rust", "🦀 Rust").trim()).toBe("Hello, 🦀 Rust! I love 🦀 Rust.");

  // 切割 / 拼接（对应 strings.Split / Join）
  expect("1,2,3".split(",")).toEqual(["1", "2", "3"]);
  expect("1 2\t3 \t4".split(/\s+/)).toEqual(["1", "2", "3", "4"]); // 按任意空白切分
  expect(["a", "b", "c"].join(",")).toBe("a,b,c");

  // 大小写 / 子串
  expect("AB大".toLowerCase() === "ab大".toLowerCase()).toBe(true); // 忽略大小写比较
  expect("rust".toUpperCase()).toBe("RUST");
  expect("查找到第一次出现的位置".slice(0, 2)).toBe("查找");
});

test("单词频率统计", () => {
  const text = "rust go rust php rust go python js";
  const counts = text.split(/\s+/).reduce<Record<string, number>>((acc, word) => {
    acc[word] = (acc[word] ?? 0) + 1; // 对应 entry().or_insert(0) += 1
    return acc;
  }, {});
  expect(counts).toEqual({ rust: 3, go: 2, php: 1, python: 1, js: 1 });
});

test("模板字符串与格式化", () => {
  const name = "laixhe";
  const age = 18;
  expect(`${name} 今年 ${age} 岁`).toBe("laixhe 今年 18 岁");
  expect(String(666).padStart(8, "0")).toBe("00000666");
});

import { test, expect } from "bun:test";

// JSON 序列化：stringify / parse / omitempty / 美化输出。
// 对应 Go golog/json_test.go 与 Rust serde_json。

test("基础序列化 / 反序列化", () => {
  const data = { name: "laixhe", age: 18, tags: ["go", "rust", "python"] };
  const s = JSON.stringify(data);
  expect(s).toBe('{"name":"laixhe","age":18,"tags":["go","rust","python"]}');

  const parsed = JSON.parse(s) as typeof data;
  expect(parsed.name).toBe("laixhe");
  expect(parsed.age).toBe(18);
  expect(parsed.tags).toEqual(["go", "rust", "python"]);
});

// omitempty 手动实现（对应 Go 的 omitempty tag）：过滤 null / 空字符串 / 空集合
function omitEmpty(obj: Record<string, unknown>): Record<string, unknown> {
  return Object.fromEntries(
    Object.entries(obj).filter(([, v]) => v !== null && v !== "" && !(Array.isArray(v) && v.length === 0)),
  );
}

test("omitempty：空值忽略", () => {
  const tJson = { time1: null, array1: [], name: "ok" };
  expect(JSON.stringify(omitEmpty(tJson))).toBe('{"name":"ok"}');

  const full = { time1: "2025-06-21T09:18:39Z", array1: [1, 2] };
  expect(JSON.stringify(full)).toBe('{"time1":"2025-06-21T09:18:39Z","array1":[1,2]}');
});

test("数值以字符串形式序列化（对应 Go string tag）", () => {
  const query = {
    path: "/index/index",
    query: "name=laixhe&age=18",
    age: "18", // 数字转字符串形式
    score: "88.8",
    is_pass: "false",
  };
  const json = JSON.stringify(query);
  expect(json).toContain('"age":"18"');
  expect(json).toContain('"score":"88.8"');
  expect(json).toContain('"is_pass":"false"');
});

test("美化输出（对应 json.MarshalIndent）", () => {
  const pretty = JSON.stringify({ name: "laixhe", age: 18 }, null, 2);
  expect(pretty).toBe('{\n  "name": "laixhe",\n  "age": 18\n}');
});

test("replacer / reviver 定制", () => {
  // replacer：序列化时剔除敏感字段（对应 Go 的 "-" tag）
  const secret = { name: "laixhe", password: "123456" };
  const safe = JSON.stringify(secret, (key, value) => (key === "password" ? undefined : value));
  expect(safe).toBe('{"name":"laixhe"}');

  // reviver：反序列化时转换数值字符串
  const parsed = JSON.parse('{"age":"19"}', (key, value) =>
    key === "age" ? Number(value) : value,
  ) as { age: number };
  expect(parsed.age).toBe(19);
  expect(typeof parsed.age).toBe("number");
});

test("解析失败抛 SyntaxError", () => {
  expect(() => JSON.parse("{invalid json")).toThrow(SyntaxError);
});

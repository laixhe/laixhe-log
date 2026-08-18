import { test, expect } from "bun:test";

// 数值进阶：格式化输出 / 溢出行为 / 类型转换。
// 对应 Rust rustlog/src/number.rs 与 Go golog 的数值示例。
//
// 前置知识：
// - JS number 是 64 位浮点（IEEE 754），整数精度上限 2^53（Number.MAX_SAFE_INTEGER）
// - 超过安全整数范围需用 BigInt（任意精度，对应 Python int / Rust 无溢出的部分）
// - parseInt/parseFloat 解析失败返回 NaN（区别于 Python 抛异常）

// 格式化输出（对应 Rust number_to_string）
test("数值格式化", () => {
  // 精度控制（四舍五入，返回字符串）
  expect((88.888).toFixed(2)).toBe("88.89");
  expect((88.0).toFixed(2)).toBe("88.00");

  // 十六进制 / 八进制 / 二进制（toString 指定进制）
  expect((666).toString(16)).toBe("29a");
  expect((666).toString(8)).toBe("1232");
  expect((666).toString(2)).toBe("1010011010");
  expect((666).toString(16).toUpperCase()).toBe("29A"); // 大写形式

  // 前导零填充 + 宽度控制（对应 {d:0>8}）
  expect(String(666).padStart(8, "0")).toBe("00000666");

  // 对齐：左对齐 / 右对齐
  expect(`left=|${"666".padEnd(10)}|`).toBe("left=|666       |");
  expect(`right=|${"666".padStart(10)}|`).toBe("right=|       666|");

  // 千分位分组（toLocaleString）
  expect((1234567).toLocaleString("en-US")).toBe("1,234,567");

  // 正负号：正数补 +，负数自带 -
  const signed = (v: number) => (v >= 0 ? `+${v}` : `${v}`);
  expect(signed(666)).toBe("+666");
  expect(signed(-888)).toBe("-888");
});

// 溢出：JS number 的精度边界与 BigInt（对应 Rust overflow / Go 溢出检查）
test("数值精度与溢出", () => {
  // 1) 精度丢失：超过 2^53 的安全整数范围后，相邻整数无法区分
  const max = Number.MAX_SAFE_INTEGER; // 2^53 - 1
  expect(Number.isSafeInteger(max)).toBe(true);
  expect(Number.isSafeInteger(max + 1)).toBe(false); // 已超出安全范围
  // 经典坑：max + 1 === max + 2（精度丢失）
  expect(max + 1 === max + 2).toBe(true);

  // 2) BigInt：任意精度整数（对应 Python int / Rust i128）
  expect(2n ** 100n).toBe(1267650600228229401496703205376n);
  expect(255n + 1n).toBe(256n); // 不回绕

  // 3) 浮点精度：0.1 + 0.2 !== 0.3（IEEE 754 经典问题）
  expect(0.1 + 0.2).not.toBe(0.3);
  expect(Math.abs(0.1 + 0.2 - 0.3)).toBeLessThan(Number.EPSILON); // 用误差比较

  // 4) 除零：JS 返回 Infinity，不抛异常（区别于 Python/Rust）
  expect(1 / 0).toBe(Infinity);
});

// 类型转换（对应 Rust type_conversion / Go strconv）
test("类型转换", () => {
  // 浮点转整数：Math.trunc 向零截断
  expect(Math.trunc(3.99)).toBe(3);
  // 四舍五入：Math.round
  expect(Math.round(3.99)).toBe(4);

  // 字符串解析：parseInt 指定进制（对应 Go strconv.ParseInt）
  expect(parseInt("666", 10)).toBe(666);
  expect(parseInt("29A", 16)).toBe(666);
  expect(parseInt("1232", 8)).toBe(666);
  expect(parseInt("1010011010", 2)).toBe(666);

  // 解析失败返回 NaN（对应 Rust 的 Err / Python 的 ValueError）
  expect(Number.isNaN(parseInt("not_a_number", 10))).toBe(true);

  // Number() 强制转换
  expect(Number("88.88")).toBe(88.88);
  expect(Number(true)).toBe(1);
  expect(Number(null)).toBe(0); // ⚠️ null 转 0，容易踩坑
});

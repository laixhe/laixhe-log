import { test, expect } from "bun:test";

// 泛型函数：用类型参数 T 保持输入输出类型一致
function identity<T>(value: T): T {
  return value;
}

test("泛型函数", () => {
  expect(identity(42)).toBe(42);
  expect(identity("hello")).toBe("hello");
});

// 泛型接口：接口带类型参数
interface Box<T> {
  value: T;
}

test("泛型接口", () => {
  const numBox: Box<number> = { value: 42 };
  const strBox: Box<string> = { value: "hi" };
  expect(numBox.value).toBe(42);
  expect(strBox.value).toBe("hi");
});

// 泛型约束：用 extends 限制类型参数必须满足某形状
function longer<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}

test("泛型约束", () => {
  expect(longer("abc", "de")).toBe("abc");
  expect(longer([1, 2], [3])).toEqual([1, 2]);
});

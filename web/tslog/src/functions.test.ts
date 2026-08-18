import { test, expect } from "bun:test";

// 函数参数与返回值类型
function add(a: number, b: number): number {
  return a + b;
}

test("参数与返回类型", () => {
  expect(add(1, 2)).toBe(3);
});

// 可选参数：用 ? 标记，调用时可省略
function greet(name: string, title?: string): string {
  return title ? `${title} ${name}` : name;
}

test("可选参数", () => {
  expect(greet("laixhe")).toBe("laixhe");
  expect(greet("laixhe", "Mr.")).toBe("Mr. laixhe");
});

// 默认值：省略时使用默认值
function power(base: number, exponent: number = 2): number {
  return base ** exponent;
}

test("默认值", () => {
  expect(power(3)).toBe(9);
  expect(power(3, 3)).toBe(27);
});

// 剩余参数：用 ... 收集多个参数为数组
function sum(...nums: number[]): number {
  return nums.reduce((acc, n) => acc + n, 0);
}

test("剩余参数", () => {
  expect(sum(1, 2, 3, 4)).toBe(10);
});

// 函数重载：同名函数声明多个签名，实现需兼容所有签名
function parse(value: string): number;
function parse(value: number): string;
function parse(value: string | number): number | string {
  if (typeof value === "string") return Number(value);
  return String(value);
}

test("函数重载", () => {
  expect(parse("42")).toBe(42);
  expect(parse(42)).toBe("42");
});

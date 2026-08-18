import { test, expect } from "bun:test";

// 原始类型：number / string / boolean / null / undefined / bigint
test("原始类型", () => {
  const num: number = 42;
  const str: string = "hello";
  const flag: boolean = true;
  const nothing: null = null;
  const undef: undefined = undefined;
  const big: bigint = 100n;

  expect(num).toBe(42);
  expect(str).toBe("hello");
  expect(flag).toBe(true);
  expect(nothing).toBeNull();
  expect(undef).toBeUndefined();
  expect(big).toBe(100n);
});

// 数组：两种写法等价
test("数组", () => {
  const nums: number[] = [1, 2, 3];
  const strs: Array<string> = ["a", "b"];

  expect(nums.length).toBe(3);
  expect(nums[1]).toBe(2);
  expect(strs).toEqual(["a", "b"]);
});

// 元组：固定长度、固定类型的数组
test("元组", () => {
  const pair: [string, number] = ["laixhe", 18];

  expect(pair[0]).toBe("laixhe");
  expect(pair[1]).toBe(18);
});

// 枚举：一组有名字的常量
// 注：现代实践也常使用「字面量联合类型」替代枚举
enum Color {
  Red,
  Green,
  Blue,
}

test("枚举", () => {
  const c: Color = Color.Green;
  expect(c).toBe(1); // 默认从 0 开始编号
});

// 联合类型：值可以是多种类型之一
test("联合类型", () => {
  let id: number | string = 1;
  id = "abc";
  expect(typeof id).toBe("string");
});

// 字面量类型：值只能是特定字面量
test("字面量类型", () => {
  let direction: "up" | "down" | "left" | "right" = "up";
  expect(direction).toBe("up");
});

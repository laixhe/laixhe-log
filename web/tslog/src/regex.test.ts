import { test, expect } from "bun:test";

// JS 内置 RegExp 支持正则，可用 /pattern/flags 字面量或 new RegExp()。

test("正则匹配手机号码", () => {
  // 中国大陆手机号：1 开头，第二位 3-9，后面跟 9 位数字，共 11 位
  const phoneRegex = /^1[3-9]\d{9}$/;

  expect(phoneRegex.test("13812345678")).toBe(true);
  expect(phoneRegex.test("19912345678")).toBe(true);
  expect(phoneRegex.test("12812345678")).toBe(false); // 第二位 2 非法
  expect(phoneRegex.test("1381234567")).toBe(false); // 少一位（10 位）
  expect(phoneRegex.test("138123456789")).toBe(false); // 多一位（12 位）
});

test("正则匹配邮箱（大小写不敏感）", () => {
  // 邮箱：本地部分 + @ + 域名 + .顶级域名
  // i 标志：忽略大小写
  const emailRegex = /^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$/i;

  expect(emailRegex.test("laixhe@example.com")).toBe(true);
  expect(emailRegex.test("LAIXHE@EXAMPLE.COM")).toBe(true);
  expect(emailRegex.test("LaixHe@Example.com")).toBe(true); // 混合大小写
  expect(emailRegex.test("user.name+tag@mail.example.org")).toBe(true);
  expect(emailRegex.test("not-an-email")).toBe(false); // 无 @
  expect(emailRegex.test("user@.com")).toBe(false); // 域名以点开头
  expect(emailRegex.test("user@example")).toBe(false); // 缺少顶级域名
});

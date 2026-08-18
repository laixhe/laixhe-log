import { test, expect } from "bun:test";

// 工具类型：TS 内置的实用类型，基于已有类型生成新类型

interface User {
  id: number;
  name: string;
  email: string;
}

// Partial<T>：所有属性变为可选
test("Partial", () => {
  const partial: Partial<User> = { name: "laixhe" };
  expect(partial.name).toBe("laixhe");
  expect(partial.id).toBeUndefined();
});

// Required<T>：所有属性变为必填
type PartialUser = { id?: number; name?: string };

test("Required", () => {
  const required: Required<PartialUser> = { id: 1, name: "laixhe" };
  expect(required.id).toBe(1);
  expect(required.name).toBe("laixhe");
});

// Readonly<T>：所有属性变为只读
test("Readonly", () => {
  const readonly: Readonly<User> = { id: 1, name: "laixhe", email: "a@b.c" };
  expect(readonly.id).toBe(1);
  // readonly.id = 2; // 编译报错
});

// Pick<T, K>：只取部分属性
test("Pick", () => {
  const picked: Pick<User, "id" | "name"> = { id: 1, name: "laixhe" };
  expect(picked.name).toBe("laixhe");
  // picked.email; // 编译报错：email 不在 Pick 结果里
});

// Omit<T, K>：去掉部分属性
test("Omit", () => {
  const omitted: Omit<User, "email"> = { id: 1, name: "laixhe" };
  expect(omitted.id).toBe(1);
  // omitted.email; // 编译报错：email 已被去掉
});

// Record<K, V>：构造键为 K、值为 V 的对象类型
test("Record", () => {
  const scores: Record<string, number> = { math: 90, english: 85 };
  expect(scores.math).toBe(90);
});

// ReturnType<T>：获取函数返回值类型
function getUser() {
  return { id: 1, name: "laixhe" };
}

test("ReturnType", () => {
  const user: ReturnType<typeof getUser> = getUser();
  expect(user.name).toBe("laixhe");
});

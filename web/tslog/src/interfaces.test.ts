import { test, expect } from "bun:test";

// interface：描述对象形状
interface User {
  name: string;
  age: number;
}

test("接口", () => {
  const user: User = { name: "laixhe", age: 18 };
  expect(user.name).toBe("laixhe");
});

// 可选属性 ? 与只读属性 readonly
interface Config {
  readonly id: number;
  name: string;
  desc?: string;
}

test("可选与只读属性", () => {
  const cfg: Config = { id: 1, name: "app" };
  expect(cfg.id).toBe(1);
  expect(cfg.desc).toBeUndefined();
  // cfg.id = 2; // 编译报错：readonly 属性不可赋值
});

// type 别名：给类型起名字，可表示联合、交叉等复杂类型
type ID = number | string;

test("类型别名", () => {
  const id: ID = "abc";
  expect(typeof id).toBe("string");
});

// 交叉类型：合并多个类型
type Name = { name: string };
type Age = { age: number };
type Person = Name & Age;

test("交叉类型", () => {
  const p: Person = { name: "laixhe", age: 18 };
  expect(p.name).toBe("laixhe");
  expect(p.age).toBe(18);
});

// 接口扩展：继承另一个接口
interface Animal {
  name: string;
}
interface Dog extends Animal {
  bark(): string;
}

test("接口扩展", () => {
  const dog: Dog = {
    name: "wangcai",
    bark: () => "woof",
  };
  expect(dog.bark()).toBe("woof");
});

import { test, expect } from "bun:test";

// Promise：表示异步操作的结果
function delay(ms: number): Promise<string> {
  return new Promise((resolve) => {
    setTimeout(() => resolve("done"), ms);
  });
}

test("Promise", async () => {
  const result = await delay(10);
  expect(result).toBe("done");
});

// async/await：异步函数的语法糖，返回 Promise
async function fetchUser(id: number): Promise<string> {
  return `user-${id}`;
}

test("async/await", async () => {
  const name = await fetchUser(1);
  expect(name).toBe("user-1");
});

// Promise.all：并行等待多个异步操作
test("Promise.all", async () => {
  const results = await Promise.all([
    fetchUser(1),
    fetchUser(2),
    fetchUser(3),
  ]);
  expect(results).toEqual(["user-1", "user-2", "user-3"]);
});

// 错误处理：异步函数出错时，await 抛出，可用 try/catch 捕获
async function risky(flag: boolean): Promise<number> {
  if (flag) throw new Error("boom");
  return 42;
}

test("异步错误处理", async () => {
  await expect(risky(true)).rejects.toThrow("boom");
  const v = await risky(false);
  expect(v).toBe(42);
});

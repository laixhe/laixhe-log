import { test, expect } from "bun:test";

// 迭代器与数组方法：map / filter / reduce / zip / flatMap / partition。
// 对应 Rust rustlog/src/iterators.rs 与 Java Stream。
//
// 对应关系速查：
// - map          → .map()
// - filter       → .filter()
// - take(n)      → .slice(0, n)；skip(n) → .slice(n)
// - enumerate    → .entries()
// - zip          → 双数组 map 配对
// - flatten      → .flat() / .flatMap()
// - fold/reduce  → .reduce()
// - partition    → .reduce() 分成两组
// - any / all    → .some() / .every()

test("map / filter / take / skip", () => {
  // map：平方（对应 1..10 平方）
  expect([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map((x) => x * x).slice(0, 3)).toEqual([1, 4, 9]);

  // filter：长度≤3 的单词
  const words = ["rust", "go", "python", "java", "c++", "js"];
  expect(words.filter((w) => w.length <= 3)).toEqual(["go", "c++", "js"]);

  // take / skip（对应 islice）
  const seq = Array.from({ length: 10 }, (_, i) => i + 1); // 1..10
  expect(seq.slice(0, 3)).toEqual([1, 2, 3]); // take(3)
  expect(seq.slice(7)).toEqual([8, 9, 10]); // skip(7)

  // step_by(5)：按索引步进取元素
  expect(seq.filter((_, i) => i % 5 === 0)).toEqual([1, 6]); // 索引 0、5
  expect(Array.from({ length: 5 }, (_, i) => i * 5)).toEqual([0, 5, 10, 15, 20]); // 对应 0..20 step 5
});

test("enumerate / zip / chain / flatten", () => {
  // enumerate：带索引遍历（对应 Rust enumerate）
  expect(["a", "b", "c"].entries()).toBeInstanceOf(Iterator); // 迭代器对象
  expect([...["a", "b", "c"].entries()]).toEqual([
    [0, "a"],
    [1, "b"],
    [2, "c"],
  ]);

  // zip：双数组一一配对
  const names = ["Alice", "Bob", "Charlie"];
  const scores = [95, 87, 92];
  expect(names.map((n, i) => [n, scores[i]])).toEqual([
    ["Alice", 95],
    ["Bob", 87],
    ["Charlie", 92],
  ]);

  // chain：拼接
  expect([1, 2, 3].concat(10, 11, 12)).toEqual([1, 2, 3, 10, 11, 12]);

  // flatten：展平嵌套数组（对应 flat_map）
  expect([[1, 2], [3, 4, 5], [6]].flat()).toEqual([1, 2, 3, 4, 5, 6]);

  // flat_map：把每个单词展开成字符
  expect(["hello", "world"].flatMap((w) => [...w])).toEqual([
    "h", "e", "l", "l", "o", "w", "o", "r", "l", "d",
  ]);
});

test("filter_map：选出合法数字", () => {
  const strs = ["123", "abc", "456", "not_a_num", "789"];
  const nums = strs.filter((s) => /^\d+$/.test(s)).map(Number);
  expect(nums).toEqual([123, 456, 789]);
});

test("reduce / sum / min / max / any / all", () => {
  const v = [3, 1, 4, 1, 5, 9, 2, 6];

  // reduce 累加（对应 fold 1..10 = 55）
  expect(Array.from({ length: 10 }, (_, i) => i + 1).reduce((acc, x) => acc + x, 0)).toBe(55);

  expect(v.reduce((a, b) => a + b, 0)).toBe(31); // sum
  expect(Math.min(...v)).toBe(1); // min
  expect(Math.max(...v)).toBe(9); // max

  expect(v.some((x) => x > 10)).toBe(false); // any > 10?
  expect(v.every((x) => x > 0)).toBe(true); // all > 0?
});

test("partition：奇偶分组", () => {
  const v = [3, 1, 4, 1, 5, 9, 2, 6];
  const { even, odd } = v.reduce<{ even: number[]; odd: number[] }>(
    (acc, x) => {
      (x % 2 === 0 ? acc.even : acc.odd).push(x);
      return acc;
    },
    { even: [], odd: [] },
  );
  expect(even).toEqual([4, 2, 6]);
  expect(odd).toEqual([3, 1, 1, 5, 9]);
});

// 综合实战：R&D 部门 30 岁以上员工的平均月薪
test("综合实战：平均月薪", () => {
  const staff = [
    { dept: "R&D", age: 28, salary: 30000 },
    { dept: "R&D", age: 35, salary: 45000 },
    { dept: "R&D", age: 42, salary: 60000 },
    { dept: "HR", age: 32, salary: 18000 },
    { dept: "R&D", age: 25, salary: 22000 },
    { dept: "Sale", age: 38, salary: 25000 },
  ];

  const query = staff
    .filter((e) => e.dept === "R&D") // 先筛选部门
    .filter((e) => e.age >= 30) // 再筛选年龄
    .map((e) => e.salary); // 提取月薪

  const avg = query.length === 0 ? 0 : query.reduce((a, b) => a + b, 0) / query.length;
  expect(avg).toBe(52500); // (45000+60000)/2 = 52500
});

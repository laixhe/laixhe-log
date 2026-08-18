import { test, expect } from "bun:test";

// 容器进阶：数组 / Map / Set / 堆 / 环形缓冲 / LRU / 去重 / 分组。
// 对应 Go golog 的 container/list、container/ring、container/heap、lru_test.go、
// slice_test.go、map_test.go、unique_test.go 与 Rust array_map.rs 的进阶部分。

// ---------- 数组操作（对应 Go slice_test.go / Rust Vec）----------
test("数组：查找 / 排序 / 去重", () => {
  // 查找（对应 slices.Contains / indexOf）
  expect([1, 2, 3].includes(2)).toBe(true);
  expect(["切片", "查找", "处理"].indexOf("查找")).toBe(1);

  // 排序（对应 slices.Sort，原地修改）
  const order = [2, 1, 3];
  order.sort((a, b) => a - b); // 升序
  expect(order).toEqual([1, 2, 3]);
  order.sort((a, b) => b - a); // 从大到小
  expect(order).toEqual([3, 2, 1]);

  // 去重（对应 unique_test.go，保持首次出现顺序）
  const nums = [3, 1, 2, 1, 3, 2, 4, 5, 4];
  expect([...new Set(nums)]).toEqual([3, 1, 2, 4, 5]);

  // 移除连续重复（对应 slices.Compact，只去除相邻重复）
  const compact = [11, 2, 2, 3, 3, 8, 11];
  const compacted = compact.filter((n, i) => i === 0 || n !== compact[i - 1]);
  expect(compacted).toEqual([11, 2, 3, 8, 11]);
});

// 结构体排序（对应 slices.SortFunc：年龄升序、相同按姓名升序）
test("对象数组排序", () => {
  const people = [
    { name: "Jax", age: 36 },
    { name: "TJ", age: 26 },
    { name: "Alex", age: 76 },
  ];
  people.sort((a, b) => a.age - b.age || a.name.localeCompare(b.name));
  expect(people.map((p) => p.name)).toEqual(["TJ", "Jax", "Alex"]);
});

// ---------- Map（对应 Go map_test.go / Rust HashMap）----------
test("Map：增删查改与有序 key", () => {
  const m = new Map<string, number>([
    ["a", 1],
    ["b", 2],
    ["c", 3],
  ]);

  // 查询
  expect(m.get("b")).toBe(2);
  expect(m.has("a")).toBe(true);

  // 删除
  m.delete("c");
  expect(m.has("c")).toBe(false);

  // 不存在则插入默认值（对应 entry().or_insert）
  if (!m.has("b")) m.set("b", 20); // 已存在，保持 2
  expect(m.get("b")).toBe(2);

  // 排序后的 key（对应 slices.Sorted(maps.Keys(m))）
  expect([...m.keys()].sort()).toEqual(["a", "b"]);
});

// ---------- Set 集合运算（对应 Rust HashSet）----------
test("Set：交集 / 并集 / 差集", () => {
  const a = new Set([1, 2, 3, 4]);
  const b = new Set([3, 4, 5, 6]);

  expect([...a].filter((x) => b.has(x))).toEqual([3, 4]); // 交集 a∩b
  expect([...new Set([...a, ...b])].sort()).toEqual([1, 2, 3, 4, 5, 6]); // 并集 a∪b
  expect([...a].filter((x) => !b.has(x))).toEqual([1, 2]); // 差集 a-b
});

// ---------- 最小堆 MinHeap（对应 Go container/heap 默认最小堆）----------
class MinHeap {
  private data: number[] = [];

  get size(): number {
    return this.data.length;
  }

  peek(): number | undefined {
    return this.data[0];
  }

  push(v: number): void {
    this.data.push(v);
    this.siftUp(this.data.length - 1);
  }

  pop(): number | undefined {
    if (this.data.length === 0) return undefined;
    const top = this.data[0];
    const last = this.data.pop()!;
    if (this.data.length > 0) {
      this.data[0] = last;
      this.siftDown(0);
    }
    return top;
  }

  private siftUp(i: number): void {
    while (i > 0) {
      const parent = (i - 1) >> 1;
      if (this.data[parent] <= this.data[i]) break;
      [this.data[parent], this.data[i]] = [this.data[i], this.data[parent]];
      i = parent;
    }
  }

  private siftDown(i: number): void {
    for (;;) {
      const left = i * 2 + 1;
      const right = left + 1;
      let smallest = i;
      if (left < this.data.length && this.data[left] < this.data[smallest]) smallest = left;
      if (right < this.data.length && this.data[right] < this.data[smallest]) smallest = right;
      if (smallest === i) break;
      [this.data[smallest], this.data[i]] = [this.data[i], this.data[smallest]];
      i = smallest;
    }
  }
}

test("最小堆：依次弹出从小到大", () => {
  const heap = new MinHeap();
  for (const n of [2, 1, 5, 3, 4]) heap.push(n);

  expect(heap.peek()).toBe(1); // 堆顶(最小)
  heap.push(0);
  expect(heap.peek()).toBe(0); // 入堆 0 后堆顶

  const popped: number[] = [];
  while (heap.size > 0) popped.push(heap.pop()!);
  expect(popped).toEqual([0, 1, 2, 3, 4, 5]); // 依次弹出（从小到大）
});

// ---------- 环形缓冲区 RingBuffer（对应 Go container/ring）----------
class RingBuffer<T> {
  private buf: (T | undefined)[];
  private head = 0;
  private size = 0;

  constructor(private capacity: number) {
    this.buf = new Array<T | undefined>(capacity).fill(undefined);
  }

  // 写入：环满则覆盖最旧数据
  add(v: T): void {
    this.buf[this.head] = v;
    this.head = (this.head + 1) % this.capacity;
    if (this.size < this.capacity) this.size++;
  }

  // 从最旧到最新遍历
  snapshot(): T[] {
    const start = (this.head - this.size + this.capacity) % this.capacity;
    const out: T[] = [];
    for (let i = 0; i < this.size; i++) {
      out.push(this.buf[(start + i) % this.capacity]!);
    }
    return out;
  }
}

test("环形缓冲区：写满覆盖最旧数据", () => {
  const ring = new RingBuffer<number>(5);
  for (let i = 0; i < 5; i++) ring.add(i); // 写入 0-4
  expect(ring.snapshot()).toEqual([0, 1, 2, 3, 4]);

  ring.add(5);
  ring.add(6);
  ring.add(7); // 覆盖最旧的 0、1、2
  expect(ring.snapshot()).toEqual([3, 4, 5, 6, 7]);
});

// ---------- LRU 缓存（对应 Go lru_test.go）----------
class LRUCache {
  // Map 保持插入顺序：队尾 = 最近使用，队首 = 最久未使用
  private cache = new Map<string, string>();

  constructor(private capacity: number) {}

  get(key: string): string {
    if (!this.cache.has(key)) return "";
    const value = this.cache.get(key)!;
    this.cache.delete(key);
    this.cache.set(key, value); // 访问后移到队尾，提升优先级
    return value;
  }

  put(key: string, value: string): void {
    if (this.cache.has(key)) this.cache.delete(key);
    this.cache.set(key, value);
    if (this.cache.size > this.capacity) {
      const oldest = this.cache.keys().next().value!; // 队首 = 最久未使用
      this.cache.delete(oldest);
    }
  }

  // 从最久未使用到最近使用遍历
  entries(): [string, string][] {
    return [...this.cache.entries()];
  }
}

test("LRU 缓存：容量淘汰最久未使用", () => {
  const cache = new LRUCache(3);
  cache.put("key1", "value1");
  cache.put("key2", "value2");
  cache.put("key3", "value3");

  expect(cache.get("key2")).toBe("value2"); // 访问 key2，变为最近使用

  cache.put("key4", "value4"); // 容量 3，淘汰 key1
  expect(cache.get("key1")).toBe(""); // key1 已被淘汰
  expect(cache.get("key2")).toBe("value2");

  // key2 再次被访问后移到队尾 → 顺序：key3, key4, key2
  expect(cache.entries().map(([k]) => k)).toEqual(["key3", "key4", "key2"]);
});

// ---------- 分组聚合（综合实战）----------
test("分组聚合", () => {
  const staff = [
    { dept: "R&D", name: "张三" },
    { dept: "R&D", name: "李四" },
    { dept: "HR", name: "王五" },
  ];

  const byDept = staff.reduce<Record<string, string[]>>((acc, e) => {
    (acc[e.dept] ??= []).push(e.name); // 对应 Collectors.groupingBy
    return acc;
  }, {});

  expect(byDept).toEqual({ "R&D": ["张三", "李四"], HR: ["王五"] });
  expect(Object.fromEntries(Object.entries(byDept).map(([k, v]) => [k, v.length]))).toEqual({
    "R&D": 2,
    HR: 1,
  });
});

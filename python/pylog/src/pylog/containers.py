"""主题：容器进阶（deque / heapq / LRU / 环形缓冲 / 去重 / 分组）。

对应 Go golog 的 container/list、container/ring、container/heap、lru_test.go、
slice_test.go、map_test.go、unique_test.go 与 Rust array_map.rs 的进阶部分。

前置知识：
- collections.deque 双端队列：两端 O(1) 增删（对应 Go container/list / Java ArrayDeque）
- heapq 最小堆：heapq.heappush / heappop（对应 Go container/heap 默认最小堆）
- collections.OrderedDict 保持插入顺序，move_to_end 可实现 LRU
- dict 去重 / 分组非常自然（key 天然唯一）
"""

from collections import OrderedDict, defaultdict, deque
from heapq import heapify, heappop, heappush


class RingBuffer:
    """环形缓冲区（对应 Go container/ring）：固定容量环，写满覆盖最旧数据。"""

    def __init__(self, capacity: int) -> None:
        self.capacity = capacity
        self.buf = [None] * capacity
        self.head = 0
        self.size = 0

    # 写入：环满则覆盖最旧数据
    def add(self, value) -> None:
        self.buf[self.head] = value
        self.head = (self.head + 1) % self.capacity
        if self.size < self.capacity:
            self.size += 1

    # 从最旧到最新遍历
    def snapshot(self) -> list:
        start = (self.head - self.size) % self.capacity
        return [self.buf[(start + i) % self.capacity] for i in range(self.size)]


class LRUCache:
    """LRU 缓存（对应 Go lru_test.go）：OrderedDict 的 move_to_end 天然实现 LRU。"""

    def __init__(self, capacity: int) -> None:
        self.capacity = capacity
        self.cache: OrderedDict = OrderedDict()

    def get(self, key: str):
        if key not in self.cache:
            return ""
        self.cache.move_to_end(key)  # 访问后移到队尾，提升优先级
        return self.cache[key]

    def put(self, key: str, value) -> None:
        if key in self.cache:
            self.cache[key] = value
            self.cache.move_to_end(key)
            return
        self.cache[key] = value
        # 超出容量时删除队首元素（最久未使用）
        if len(self.cache) > self.capacity:
            self.cache.popitem(last=False)

    # 从最久未使用到最近使用遍历
    def entries(self) -> list:
        return list(self.cache.items())

    def show(self) -> None:
        for k, v in self.cache.items():
            print(f"{k}={v}")


def run() -> None:
    print("========== 容器进阶 ==========")

    # ---------- 1. 双端队列 deque（对应 Go container/list）----------
    dq = deque([2, 3, 4])
    dq.appendleft(1)  # 头部插入（push_front）
    dq.append(5)      # 尾部插入（push_back）
    print("appendleft/append 后:", list(dq))  # [1, 2, 3, 4, 5]

    front = dq.popleft()  # 头部弹出（pop_front）
    back = dq.pop()       # 尾部弹出（pop_back）
    print(f"popleft={front} pop={back} 剩余: {list(dq)}")  # 1 5 [2, 3, 4]

    # 用 deque 模拟队列（FIFO）
    queue = deque(["a", "b"])
    print("队首元素:", queue.popleft())  # a

    # ---------- 2. 堆 heapq（对应 Go container/heap，默认最小堆）----------
    h = [2, 1, 5, 3, 4]
    heapify(h)  # 建堆 O(n)
    print("堆顶(最小):", h[0])  # 1

    heappush(h, 0)  # 入堆
    print("入堆 0 后堆顶:", h[0])  # 0

    # 依次弹出（从小到大）
    print("依次弹出:", [heappop(h) for _ in range(len(h))])  # [0,1,2,3,4,5]

    # 最大堆：取负数包装（对应 Rust Reverse）
    max_heap = [-x for x in [30, 10, 50, 20]]
    heapify(max_heap)
    print("最大堆依次弹出:", [-heappop(max_heap) for _ in range(len(max_heap))])  # [50,30,20,10]

    # ---------- 3. 环形缓冲区（对应 Go container/ring）----------
    ring = RingBuffer(5)
    for i in range(5):
        ring.add(i)  # 写入 0-4
    print("环遍历:", ring.snapshot())  # [0,1,2,3,4]

    ring.add(5)
    ring.add(6)
    ring.add(7)  # 覆盖最旧的 0、1、2
    print("覆盖后环遍历:", ring.snapshot())  # [3,4,5,6,7]

    # ---------- 4. LRU 缓存（对应 Go lru_test.go）----------
    cache = LRUCache(3)
    cache.put("key1", "value1")
    cache.put("key2", "value2")
    cache.put("key3", "value3")
    cache.show()  # key1 key2 key3（key1 最久未使用）

    print("get key2 →", cache.get("key2"))  # value2
    cache.show()  # key1 key3 key2（key2 被移到队尾）

    cache.put("key4", "value4")  # 容量 3，淘汰 key1
    cache.show()  # key3 key2 key4

    # ---------- 5. 切片处理（对应 Go slice_test.go）----------
    # 查找
    print(2 in [1, 2, 3])  # True
    print("查找" in ["切片", "查找", "处理"])  # True

    # 排序（原地 / 新列表）
    order = [2, 1, 3]
    order.sort()  # 升序（原地）
    print(order)  # [1, 2, 3]
    order.sort(reverse=True)  # 从大到小
    print(order)  # [3, 2, 1]

    # 移除连续重复（对应 slices.Compact，只去除相邻重复）
    compact = [11, 2, 2, 3, 3, 8, 11]
    compacted = [n for i, n in enumerate(compact) if i == 0 or n != compact[i - 1]]
    print(compacted)  # [11, 2, 3, 8, 11]

    # 结构体排序（年龄升序、相同按姓名升序）
    people = [
        {"name": "Jax", "age": 36},
        {"name": "TJ", "age": 26},
        {"name": "Alex", "age": 76},
    ]
    people.sort(key=lambda p: (p["age"], p["name"]))  # 对应 slices.SortFunc
    # 结果：[{'name': 'TJ', 'age': 26}, {'name': 'Jax', 'age': 36}, {'name': 'Alex', 'age': 76}]
    print(people)

    # ---------- 6. 字典处理（对应 Go map_test.go）----------
    m = {"a": 1, "b": 2, "c": 3}
    keys = list(m.keys())
    print(keys)  # ['a', 'b', 'c']（插入序）

    sorted_keys = sorted(m.keys())
    print(sorted_keys)  # ['a', 'b', 'c']（排序后）

    # ---------- 7. 去重（对应 Go unique_test.go）----------
    nums = [3, 1, 2, 1, 3, 2, 4, 5, 4]
    print("dict.fromkeys 去重:", list(dict.fromkeys(nums)))  # [3, 1, 2, 4, 5]（保持顺序）
    print("set 去重(无序):", sorted(set(nums)))  # [1, 2, 3, 4, 5]

    # ---------- 8. 分组聚合（综合实战）----------
    staff = [
        {"dept": "R&D", "name": "张三"},
        {"dept": "R&D", "name": "李四"},
        {"dept": "HR", "name": "王五"},
    ]
    by_dept: dict[str, list] = defaultdict(list)
    for e in staff:
        by_dept[e["dept"]].append(e["name"])  # 对应 Java groupingBy
    print("按部门分组:", dict(by_dept))  # {'R&D': ['张三', '李四'], 'HR': ['王五']}

    count = {dept: len(names) for dept, names in by_dept.items()}
    print("各部门人数:", count)  # {'R&D': 2, 'HR': 1}

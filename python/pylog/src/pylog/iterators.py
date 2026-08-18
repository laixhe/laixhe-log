"""主题：迭代器与推导式（map / filter / zip / itertools / 生成器）。

对应 Rust rustlog/src/iterators.rs 与 Java StreamDemo。

前置知识：
- Python 迭代器是「惰性求值」的：for 循环本质是 iter() + next()
- 列表推导式 [x*x for x in ...] 是最常用的「map + collect」
- 生成器（yield）是惰性迭代器，处理大数据流时节省内存
- itertools 提供 take/skip/chain/flatten 等迭代器适配器
"""

from collections.abc import Iterator
from functools import reduce
from itertools import chain, islice

# 综合实战使用的员工数据（部门，年龄，月薪）
STAFF = [
    {"dept": "R&D", "age": 28, "salary": 30000},
    {"dept": "R&D", "age": 35, "salary": 45000},
    {"dept": "R&D", "age": 42, "salary": 60000},
    {"dept": "HR", "age": 32, "salary": 18000},
    {"dept": "R&D", "age": 25, "salary": 22000},
    {"dept": "Sale", "age": 38, "salary": 25000},
]


def run() -> None:
    print("========== 迭代器与推导式 ==========")

    # ---------- 1. 三种「遍历」方式 ----------
    v = [10, 20, 30]

    # for 循环遍历（最常用）
    print("for: ", end="")
    for x in v:
        print(x, end=" ")
    print()

    # 索引遍历（range + len）
    for i in range(len(v)):
        print(f"索引 {i} = {v[i]}")

    # 带索引遍历（对应 enumerate）
    for i, x in enumerate(v):
        print(f"enumerate: [{i}] = '{x}'")

    # 手动迭代（展示迭代器底层）
    it = iter(v)
    print("手动 next:", next(it), next(it), next(it), "结束?",
          next(it, None) is None)  # 10 20 30 结束? True

    # ---------- 2. 中间操作（推导式 / itertools）----------
    # map：对每个元素做变换
    squares = [x * x for x in range(1, 11)]
    print("1..10 平方:", squares)  # [1,4,9,16,...,100]

    # filter：只保留满足条件的元素
    words = ["rust", "go", "python", "java", "c++", "js"]
    short_words = [w for w in words if len(w) <= 3]
    print("长度≤3 的单词:", short_words)  # ['go', 'c++', 'js']

    # filter_map 二合一：把能解析为数字的挑出来
    strs = ["123", "abc", "456", "not_a_num", "789"]
    nums = [int(s) for s in strs if s.isdigit()]
    print("filter_map 选出合法数字:", nums)  # [123, 456, 789]

    # take(n) / skip(n)：itertools.islice
    seq = range(1, 11)
    print("take(3):", list(islice(seq, 3)))    # [1,2,3]
    print("skip(7):", list(islice(seq, 7, None)))  # [8,9,10]

    # step_by(n)：步进切片
    print("step_by(5) 0..20:", list(range(0, 21, 5)))  # [0,5,10,15,20]

    # zip：把两个迭代器一一配对（长度以较短为准）
    names = ["Alice", "Bob", "Charlie"]
    scores = [95, 87, 92]
    print("zip 配对:", list(zip(names, scores)))
    # [('Alice', 95), ('Bob', 87), ('Charlie', 92)]

    # chain：把两个迭代器首尾相接
    print("chain:", list(chain(range(1, 4), range(10, 13))))  # [1,2,3,10,11,12]

    # flatten：把嵌套列表展平一层
    nested = [[1, 2], [3, 4, 5], [6]]
    print("flatten:", [x for sub in nested for x in sub])  # [1,2,3,4,5,6]

    # 把每个单词的字符展开（flat_map）
    chars = [ch for w in ["hello", "world"] for ch in w]
    print("flat_map 展开字符:", chars)  # ['h','e','l','l','o','w','o','r','l','d']

    # ---------- 3. 终结操作（真正触发计算）----------
    v8 = [3, 1, 4, 1, 5, 9, 2, 6]

    # sum / count / min / max
    print("sum=", sum(v8), " count=", len(v8), " min=", min(v8), " max=", max(v8))
    # sum= 31  count= 8  min= 1  max= 9

    # any / all：是否「有一个」/「全部」满足条件
    print("any > 10?", any(x > 10 for x in v8))  # False
    print("all > 0? ", all(x > 0 for x in v8))   # True

    # reduce：累积聚合（对应 fold / reduce）
    fold_sum = reduce(lambda acc, x: acc + x, range(1, 11), 0)
    print("reduce 累加 1..10 =", fold_sum)  # 55

    # partition：按条件分成两组
    even = [x for x in v8 if x % 2 == 0]
    odd = [x for x in v8 if x % 2 != 0]
    print("partition 奇偶分: 偶=", even, " 奇=", odd)  # [4,2,6] [3,1,1,5,9]

    # ---------- 4. 生成器（惰性迭代，对应 Rust 迭代器的惰性）----------
    def fibonacci() -> Iterator[int]:
        a, b = 0, 1
        while True:
            yield a  # 每次 next() 才计算一次
            a, b = b, a + b

    gen = fibonacci()
    print("斐波那契前 5 个:", [next(gen) for _ in range(5)])  # [0, 1, 1, 2, 3]

    # ---------- 5. 综合实战：R&D 部门 30 岁以上平均月薪 ----------
    query = [e["salary"] for e in STAFF if e["dept"] == "R&D" and e["age"] >= 30]
    avg = sum(query) / len(query) if query else 0
    print(f"R&D 30+ 员工平均月薪: {avg:.0f} 元/月")  # (45000+60000)/2 = 52500

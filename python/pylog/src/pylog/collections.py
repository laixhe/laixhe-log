"""主题：集合类型（list / tuple / dict / set + 列表推导式）。"""


def run() -> None:
    print("========== 集合类型 ==========")

    # 列表 list：可变、有序
    lst = [1, 2, 3]
    lst.append(4)           # 追加元素
    print(lst)              # [1, 2, 3, 4]
    print(lst[0], lst[-1])  # 1 4（支持索引和负数索引）

    # 元组 tuple：不可变、有序
    tup = (1, 2, 3)
    print(tup[0])  # 1

    # 字典 dict：键值对
    d = {"name": "laixhe", "age": 18}
    print(d["name"])  # laixhe
    for k, v in d.items():
        print(f"  {k} -> {v}")

    # 集合 set：无序、去重
    st = {1, 2, 2, 3}
    print(st)  # {1, 2, 3}

    # 列表推导式
    squares = [x * x for x in range(5)]
    print(squares)  # [0, 1, 4, 9, 16]

"""主题：控制流（if / for / while）。"""


def run() -> None:
    print("========== 控制流 ==========")

    # if / elif / else
    x = 10
    if x > 0:
        print("正数")
    elif x < 0:
        print("负数")
    else:
        print("零")

    # for 循环（range）
    for i in range(5):
        print(i, end=" ")  # 0 1 2 3 4
    print()

    # for 遍历列表
    for item in ["a", "b", "c"]:
        print(item, end=" ")  # a b c
    print()

    # while 循环
    n = 0
    while n < 3:
        print(n, end=" ")  # 0 1 2
        n += 1
    print()

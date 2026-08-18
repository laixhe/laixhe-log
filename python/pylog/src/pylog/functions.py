"""主题：函数（定义 / 默认参数 / 变长参数 / lambda）。"""


def run() -> None:
    print("========== 函数 ==========")

    # 基本函数
    def add(a, b):
        return a + b

    print(add(1, 2))  # 3

    # 默认参数
    def greet(name="world"):
        print(f"hello {name}")

    greet()          # hello world
    greet("pylog")   # hello pylog

    # 变长参数：*args 接收位置参数（元组），**kwargs 接收关键字参数（字典）
    def fn(*args, **kwargs):
        print(args, kwargs)

    fn(1, 2, x=3, y=4)  # (1, 2) {'x': 3, 'y': 4}

    # lambda 匿名函数（通常作为参数内联使用，而不是赋值给变量）
    nums = [3, 1, 2]
    print(sorted(nums, key=lambda x: -x))  # [3, 2, 1]（按降序）

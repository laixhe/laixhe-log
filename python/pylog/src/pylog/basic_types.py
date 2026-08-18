"""主题：变量与基本数据类型（int / float / bool / str / None + 类型转换）。"""


def run() -> None:
    print("========== 基本数据类型 ==========")

    # 整数 int
    i = 10
    # 浮点数 float
    f = 3.14
    # 布尔 bool
    b = True
    # 字符串 str
    s = "hello"
    # 空值 None
    n = None

    print(i, f, b, s, n)  # 10 3.14 True hello None

    # type() 查看类型
    print(type(i), type(f), type(b), type(s), type(n))
    # <class 'int'> <class 'float'> <class 'bool'> <class 'str'> <class 'NoneType'>

    # 类型转换
    print(int("42"))     # 42
    print(float("3.14"))  # 3.14
    print(str(123))      # "123"
    print(bool(0))       # False（0 为假，非 0 为真）

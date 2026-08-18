"""主题：数值类型进阶（格式化 / 溢出 / 类型转换）。

对应 Rust rustlog/src/number.rs 与 Go golog 的数值示例。

前置知识：
- Python 的 int 是任意精度（无溢出），与 Go/Rust/Java 的固定位数整数完全不同
- float 遵循 IEEE 754，格式化时注意四舍五入
- 类型转换：int() / float() 解析失败会抛 ValueError（区别于 PHP 的宽松转换）
"""


def run() -> None:
    print("========== 数值类型进阶 ==========")

    # ---------- 1. 格式化输出 ----------
    i = 666
    f1 = 88.888
    f2 = 88.0

    # 精度控制（四舍五入）
    print(f"f1={f1:.2f}")  # f1=88.89
    print(f"f2={f2:.2f}")  # f2=88.00

    # 十六进制 / 八进制 / 二进制
    print(f"666 hex=0x{i:X}  octal=0o{i:o}  binary=0b{i:b}")
    # 666 hex=0x29A  octal=0o1232  binary=0b1010011010

    # 前导零填充 + 宽度控制
    print(f"666 with leading zeros: {i:08d}")  # 00000666

    # 对齐：< 左对齐，^ 居中，> 右对齐（默认）
    print(f"left=|{i:<10}| center=|{i:^10}| right=|{i:>10}|")
    # left=|666       | center=|   666    | right=|       666|

    # 正负号显式显示
    print(f"positive={666:+}  negative={-888:+}")  # positive=+666  negative=-888

    # 千分位分组
    print(f"grouping: {1234567:,}")  # 1,234,567

    # ---------- 2. 溢出：Python 整数无上限 ----------
    x = 255
    print(f"255 + 1 = {x + 1}（Python int 任意精度，不回绕）")  # 256
    print(f"2**100 = {2**100}（任意大整数）")
    # 2**100 = 1267650600228229401496703205376

    # 对比：Go/Rust 的 u8 溢出、Java 的 addExact 抛异常 —— Python 都不存在
    print("Python int 无溢出，无需 checked/saturating/wrapping 处理")
    print(f"float 溢出: {1e308 * 10}（float 溢出为 inf，区别于 int）")  # inf

    # 除零抛 ZeroDivisionError（对应 Java ArithmeticException / PHP DivisionByZeroError）
    try:
        1 / 0
    except ZeroDivisionError:
        print("1 / 0 = 抛 ZeroDivisionError")

    # ---------- 3. 类型转换 ----------
    print(int("666"))        # 666
    print(float("88.88"))    # 88.88
    print(int(3.99))         # 3（向零截断）
    print(round(3.99))       # 4（四舍五入）
    print(bin(666), oct(666), hex(666))  # 0b1010011010 0o1232 0x29a

    # 解析失败抛 ValueError（区别于 PHP 返回 0）
    try:
        int("not_a_number")
    except ValueError:
        print("int('not_a_number') = 抛 ValueError")

    # 严格的进制解析（对应 Go strconv.ParseInt 指定 base）
    print(int("29A", 16))  # 666
    print(int("1232", 8))  # 666
    print(int("1010011010", 2))  # 666

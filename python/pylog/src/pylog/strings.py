"""主题：字符串（f-string / 常用方法 / 切片）。"""


def run() -> None:
    print("========== 字符串 ==========")

    # f-string 格式化
    name = "laixhe"
    age = 18
    print(f"{name} 今年 {age} 岁")

    # 常用方法
    s = "hello world"
    print(s.upper())           # HELLO WORLD
    print(s.split())           # ['hello', 'world']
    print(s.replace("world", "python"))  # hello python
    print(s.startswith("he"))  # True
    print(" world ".strip())   # world（去两端空白）

    # 切片 [start:end:step]
    s2 = "0123456789"
    print(s2[0:5])    # 01234
    print(s2[::-1])   # 9876543210（反转）

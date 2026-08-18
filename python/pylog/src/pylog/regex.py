"""主题：正则表达式（re 标准库）。"""

import re


def run() -> None:
    print("========== 正则 ==========")

    # 手机号：1 开头，第二位 3-9，后面 9 位数字，共 11 位
    phone_re = re.compile(r"^1[3-9]\d{9}$")
    phones = ["13812345678", "19912345678", "12812345678", "1381234567"]
    for p in phones:
        print(f"{p}: {'匹配' if phone_re.match(p) else '不匹配'}")

    # 邮箱：大小写不敏感用 re.IGNORECASE（等价于模式里写 (?i)）
    email_re = re.compile(
        r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$",
        re.IGNORECASE,
    )
    emails = [
        "laixhe@example.com",
        "LAIXHE@EXAMPLE.COM",
        "LaixHe@Example.com",
        "user.name+tag@mail.example.org",
        "not-an-email",
    ]
    for e in emails:
        print(f"{e}: {'匹配' if email_re.match(e) else '不匹配'}")

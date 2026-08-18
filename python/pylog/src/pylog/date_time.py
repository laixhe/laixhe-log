"""主题：时间与日期（datetime / time 标准库）。"""

import time
from datetime import datetime


def run() -> None:
    print("========== 时间与日期 ==========")

    # 1. 当前时间格式化
    # strftime 格式：%Y 年(4位) %m 月 %d 日 %H 时(24h) %M 分 %S 秒
    now = datetime.now()
    print("当前时间格式化:", now.strftime("%Y-%m-%d %H:%M:%S"))  # 2026-08-13 12:13:14

    # 2. 获取当前时间戳（秒级）
    ts = int(time.time())
    print("当前时间戳(秒):", ts)

    # 3. 时间戳转时间对象（本地时区）
    dt = datetime.fromtimestamp(ts)
    print("时间戳转时间对象:", dt.strftime("%Y-%m-%d %H:%M:%S"))

    # 4. 时间字符串解析
    parsed = datetime.strptime("2026-08-13 12:13:14", "%Y-%m-%d %H:%M:%S")
    print("字符串解析:", parsed.strftime("%Y-%m-%d %H:%M:%S"))

    # 5. 时间比较
    t1 = datetime.strptime("2026-08-13 12:13:14", "%Y-%m-%d %H:%M:%S")
    t2 = datetime.strptime("2026-08-13 12:13:15", "%Y-%m-%d %H:%M:%S")
    print("t1 < t2:", t1 < t2)  # True
    print("t1 == t2:", t1 == t2)  # False
    print("时间差(秒):", (t2 - t1).total_seconds())  # 1.0

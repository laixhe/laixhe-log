"""主题：异常处理（try / except / finally / 自定义异常）。"""


def run() -> None:
    print("========== 异常处理 ==========")

    # try / except / finally
    try:
        1 / 0  # 触发除零异常
    except ZeroDivisionError as e:
        print("除零错误:", e)
    finally:
        print("无论是否异常都会执行")

    # 自定义异常
    class MyError(Exception):
        pass

    try:
        raise MyError("自定义异常")
    except MyError as e:
        print("捕获到:", e)

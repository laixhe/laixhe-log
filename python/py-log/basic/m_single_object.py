import threading


# 单例模式
class SingleObject:
    _instance = None           # 保存单例对象
    _lock = threading.RLock()  # 锁

    # 先调用 __new__ 后再调用 __init__
    def __new__(cls, *args, **kwargs):

        if cls._instance:  # 如果已经有单例了就不再去抢锁，避免IO等待
            return cls._instance

        with cls._lock:    # 使用with语法，方便抢锁释放锁
            if not cls._instance:
                cls._instance = super().__new__(cls, *args, **kwargs)
            return cls._instance
        pass

    pass


def m_single_object_run():
    s1 = SingleObject()
    print(id(s1))  # 2887550392112
    s1.age = 10

    s2 = SingleObject()
    print(id(s2))  # 2887550392112
    print(s2.age)  # 10

    s3 = SingleObject()
    print(id(s3))  # 2887550392112
    print(s3.age)  # 10
    pass

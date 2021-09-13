# 单例模式
class SingleObject:
    # 先调用 __new__ 后再调用 __init__
    def __new__(cls, *args, **kwargs):
        # 判断是否存在这个属性
        if not hasattr(cls, '_instance'):
            cls._instance = super().__new__(cls, *args, **kwargs)
            #
            cls.age: int = 0
        return cls._instance

    pass


if __name__ == '__main__':
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

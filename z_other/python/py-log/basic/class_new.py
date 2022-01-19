# 面向对象的三个基本特征：封装(Encapsulation)、继承(Inheritance)、多态(Polymorphism)

# 由关键字```class```声明，没有 public、protected、private 语法关键字，__ 两个下划线开头代表 private 
# 默认情况下，成员是公共的，没有 interface 接口修饰，也没有抽象类 abstract

# 特殊方法
# __new__(cls, *args, **kwargs) 在 __init__ 之前运行
# __init__    构造方法，在生成对象时调用
# __del__     析构函数，释放对象时使用
# __call__    函数调用
# __setitem__ 按照索引赋值
# __getitem__ 按照索引获取值


def class_run():
    base = BaseClass(18, 'laixhe')
    base.show()               # 结果： base show uid=18, name=laixhe
    test_base = TestInterface()
    test_base.show()          # 结果： test base show

    show_interface(base)       # 结果： base show uid=18, name=laixhe
    show_interface(test_base)  # 结果： test base show

    # 类属性、方法(静态)
    print(BaseClass.age)  # 结果：0
    BaseClass.age += 1
    print(BaseClass.age)  # 结果：1
    BaseClass.age += 1
    print(BaseClass.age)  # 结果：2
    age: int = BaseClass.get_age()
    print(age)            # 结果：2
    pass


# 定义类， 用于继承
class BaseInterface:
    def show(self):
        pass


# 定义类，并继承
class BaseClass(BaseInterface):
    
    age: int = 0  # 类属性

    # 构造方法
    def __init__(self, _uid: int, _name: str):
        self.uid: int = _uid      # 对象实例属性
        self.name: str = _name
        self.__like: str = '爱好'  # 加两个下划线 将此属性私有化
        pass

    def show(self):
        print("base show uid=%d, name=%s" % (self.uid, self.name))
        pass
    
    # 类方法(静态方法)
    @classmethod
    def get_age(cls) -> int:
        return cls.age
    
    # 加两个下划线 将此方法私有化
    def __get_name(self) -> str:
        return self.name
    
    # 析构方法
    def __del__(self):
        pass


# 定义类，并继承
class TestInterface(BaseInterface):
    def show(self):
        print('test base show')
        pass


# 利用类实现多态
def show_interface(b: BaseInterface):
    b.show()
    pass

# 面向对象的三个基本特征：封装(Encapsulation)、继承(Inheritance)、多态(Polymorphism)

# 由关键字```class```声明，没有 public、protected、private 语法关键字，__ 两个下划线开头代表 private 
# 默认情况下，成员是公共的，没有 interface 接口修饰，也没有抽象类 abstract

#

# 定义类， 用于继承
class BaseInterface:
    def Show(self):
        pass

# 定义类，并继承
class Base(BaseInterface):
    
    age: int = 0  # 类属性

    # 构造方法
    def __init__(self, _uid: int, _name: str):
        self.uid: int = _uid      # 对象实例属性
        self.name: str = _name
        self.__like: str = '爱好'  # 加两个下划线 将此属性私有化

    def Show(self):
        print("base show uid=%d, name=%s" % (self.uid, self.name))
    
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
class TestBase(BaseInterface):
    def Show(self):
        print('test base show')

# 利用类实现多态
def Show(b: BaseInterface):
    b.Show()

if __name__ == '__main__':
    base = Base(18, 'laixhe')
    base.Show()      # 结果： base show uid=18, name=laixhe
    testBase = TestBase()
    testBase.Show()  # 结果： test base show

    Show(base)      # 结果： base show uid=18, name=laixhe
    Show(testBase)  # 结果： test base show
    
    # 类属性、方法(静态)
    print(Base.age)  # 结果：0
    Base.age += 1
    Base.age += 1
    age: int = Base.get_age()
    print(age)  # 结果：2


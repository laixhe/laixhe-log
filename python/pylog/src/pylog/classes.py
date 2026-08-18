"""主题：类与对象（构造方法 / 实例方法 / 继承）。"""


def run() -> None:
    print("========== 类与对象 ==========")

    class Person:
        # 构造方法
        def __init__(self, name, age):
            self.name = name
            self.age = age

        # 实例方法
        def say(self):
            print(f"我是 {self.name}，今年 {self.age} 岁")

    p = Person("laixhe", 18)
    p.say()  # 我是 laixhe，今年 18 岁

    # 继承
    class Student(Person):
        def study(self):
            print(f"{self.name} 在学习")

    s = Student("xiaoming", 20)
    s.say()    # 我是 xiaoming，今年 20 岁（继承自父类）
    s.study()  # xiaoming 在学习

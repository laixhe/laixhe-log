from enum import Enum, unique

# 没有常量，没有枚举但枚举模块，可通过用 class 关键字，继承 Enum 类
# 在 python 中，没有一个专门的语法代表常量，程序员约定俗成用变量名全部大写代表常量


# 枚举
# 使用修饰器@unique，则值也唯一不可重复
@unique
class Color(int, Enum):
    read = 0
    green = 1
    blue = 2
    pass


def const_enum_run():
    print(Color.blue, Color.blue.name, Color.blue.value)  # 结果： Color.blue blue 2
    pass

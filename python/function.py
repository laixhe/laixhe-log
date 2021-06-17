# 由关键字```def```声明
# 有默认参数值
# 有可变长参数(不定长参数)

# 函数的参数传递是通过对象引用来完成的，在内部修改是否影响外部数据要看参数是否是可变类型(list dict)

#

# 无参数，无返回值
def Void():
    print('void')


# 可变长参数(不定长参数)，参数会以元组(tuple)的形式导入
def Names(*s: str):
    print(s, len(s))


# 有参数和默认值，有返回值
def Test(a: int, b: bool = False, s: str = 'str...') -> int:
    print(a, b, s)
    return a


# 关键字可变参数(不定长参数)，参数会以字典(dict)的形式导入
def keyFunc(**kwargs: dict):
    print(kwargs)


if __name__ == '__main__':
    Void()                # 结果： void
    Names('a', 'b', 'c')  # 结果： ('a', 'b', 'c') 3
    t: int = Test(1)      # 结果： 1 False str...
    print(t)              # 结果： 1
    
    dicts = {'name': 'laixhe', 'age': 18}
    keyFunc(**dicts)  # 结果： {'name': 'laixhe', 'age': 18}
    keyFunc(name='laixhe-1', age=19)  # 结果： {'name': 'laixhe-1', 'age': 19}

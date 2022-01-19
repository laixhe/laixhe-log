# 由关键字```def```声明
# 有默认参数值
# 有可变长参数(不定长参数)

# 函数的参数传递是通过对象引用来完成的，在内部修改是否影响外部数据要看参数是否是可变类型(list dict)

def func_run():
    func_void()  # 结果： void
    func_names('a', 'b', 'c')  # 结果： ('a', 'b', 'c') 3

    t: int = func_args(1)  # 结果： 1 False str...
    print(t)  # 结果： 1

    dicts = {'name': 'laixhe', 'age': 18}
    func_key(**dicts)  # 结果： {'name': 'laixhe', 'age': 18}
    pass


# 无参数，无返回值
def func_void():
    print('void')
    pass


# 可变长参数(不定长参数)，参数会以元组(tuple)的形式导入
def func_names(*s: str):
    print(s, len(s))
    pass


# 有参数和默认值，有返回值
def func_args(a: int, b: bool = False, s: str = 'str...') -> int:
    print(a, b, s)
    return a


# 关键字可变参数(不定长参数)，参数会以字典(dict)的形式导入
def func_key(**kwargs: dict):
    print(kwargs)
    pass


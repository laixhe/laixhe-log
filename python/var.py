# 要注意 python 本身没有类型声明，会根据变量的值，自动把变量转换为正确的数据类型 
# (在执行间期可以改变为其它类型) ，尽量使用 python3.7 以上版本

if __name__ == '__main__':
    world = 'World'
    print(f'Hello, {world}')
    print('Hello, %s' % world)
    # 结果： Hello, World
    
    a: int = 1
    b: str = '中文'
    c: bool = False
    d: float = 1.1
    # 结果： <class 'int'> 1 <class 'str'> 中文 <class 'bool'> False <class 'float'> 1.1
    print(type(a), a, type(b), b, type(c), c, type(d), d)

    # 虽然上面标明了数据的类型，但还是可以在执行期间变改数据的类型
    a = '中文'
    b = 1
    c = 1.1
    d = True
    # 结果： <class 'str'> 中文 <class 'int'> 1 <class 'float'> 1.1 <class 'bool'> True
    print(type(a), a, type(b), b, type(c), c, type(d), d)

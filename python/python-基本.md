# Python 基本语法

个人 Python 基础语法速查。

## 第一个例子

```python
if __name__ == '__main__':
    print('...main...')
```

`if __name__ == '__main__'` 用于判断当前文件是被直接运行还是被 import，直接运行时才执行下面的代码。

## 变量与数据类型

```python
# 数值
i = 10          # 整数 int
f = 3.14        # 浮点数 float
b = True        # 布尔 bool

# 字符串 str
s = "hello"

# 列表 list（可变、有序）
lst = [1, 2, 3]

# 元组 tuple（不可变、有序）
tup = (1, 2, 3)

# 字典 dict（键值对）
d = {"name": "laixhe", "age": 18}

# 集合 set（无序、去重）
st = {1, 2, 3}

# 空值 None
n = None
```

## 控制流

```python
# if / elif / else
x = 10
if x > 0:
    print("正数")
elif x < 0:
    print("负数")
else:
    print("零")

# for 循环
for i in range(5):
    print(i)  # 0 1 2 3 4

# while 循环
n = 0
while n < 3:
    print(n)
    n += 1
```

## 函数

```python
def add(a, b):
    return a + b

# 默认参数
def greet(name="world"):
    print(f"hello {name}")

# 元组变长参数 *args
# 字典关键参数 **kwargs
def fn(*args, **kwargs):
    print(args)    # 位置参数 -> 元组
    print(kwargs)  # 关键字参数 -> 字典

fn(1, 2, x=3, y=4)  # (1, 2) {'x': 3, 'y': 4}
```

## 类

```python
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
```

## 异常处理

```python
try:
    x = 1 / 0
except ZeroDivisionError as e:
    print("除零错误:", e)
finally:
    print("无论是否异常都会执行")
```

## 常用内置函数

```python
print(len([1, 2, 3]))            # 长度 -> 3
print(type(1))                   # 类型 -> <class 'int'>
print(range(3))                  # 范围 -> range(0, 3)
print(list(enumerate([1, 2])))   # 枚举 -> [(0, 1), (1, 2)]
print(list(zip([1, 2], [3, 4]))) # 配对 -> [(1, 3), (2, 4)]
print(sorted([3, 1, 2]))         # 排序 -> [1, 2, 3]
```

### 类型
- Number（数字）
  - int：有符号整型
  - long：长整型，也可以代表八进制和十六进制（Python3中没有）
  - float：浮点型
  - complex：复数
- String（字符串）
- Boolean（布尔类型）
  - True 真
  - False 假
- List（列表）
- Tuple（元组）
- Set（集合）
- Dictionary（字典）

`不管对于多大或者多小的整数，Python 3.x 只用 int 一种类型存储，表示为长整型，并且整数的取值范围是无限的。`

### 常用的类型转换函数

```
int(x)   把 x 转换为整数
float(x) 把 x 转换为浮点数
str(x)   把 x 转换为字符串类型
list(x)  把 x 转换为列表类型
chr(x)   把 x 转换为一个字符
ord(x)   把字符 x 转换为相应整数值
hex(x)   把整数 x 转换为十六进制字符串
oct(x)   把整数 x 转换为八进制字符串
```

### List（列表）
- 列表是最常用的 Python 数据类型，它可以作为一个方括号内的逗号分隔值出现。在其它语言里叫做数组。
- [int, int]

常用方法
```
len(list)——列表元素个数
list(seq)——将元组转换为列表
max(list)——返回列表元素最大值
min(list)——返回列表元素最小值
list.append(obj)——在列表末尾添加新的对象
list.insert(index, obj)——在列表头部添加新的对象
list.pop([index=-1])——移除列表中的一个元素（默认最后一个元素），并且返回该元素的值
list.remove(obj)——移除列表中某个值的第一个匹配项
list.reverse()——反向列表中元素
list.sort( key=None, reverse=False)——对原列表进行排序
list.clear()——清空列表
```

### Dictionary（字典）
- 字典的每个键值 key=>value 对用冒号 : 分割，每个对之间用逗号(,)分割，整个字典包括在花括号 {} 中
- {key1 : value1, key2 : value2, key3 : value3 }

常用方法
```
len(dict)——计算字典元素个数，即键的总数。
str(dict)——输出字典，可以打印的字符串表示。
type(variable)——返回输入的变量类型，如果变量是字典就返回字典类型。
dict.clear()——删除字典内所有元素。
dict.get(key, default=None)——返回指定键的值，如果键不在字典中返回 default 设置的默认值
key in dict——如果键在字典dict里返回true，否则返回false。
dict.items()——以列表返回一个视图对象。
dict.keys()——返回一个视图对象。
dict.values()——返回一个视图对象。
pop(key[,default])——删除字典 key（键）所对应的值，返回被删除的值。
```

### Tuple（元组）
- Python 的元组与列表类似，不同之处在于元组的元素不能修改。
- 元组使用小括号( )，列表使用方括号 [ ]。
- 元组创建很简单，只需要在括号中添加元素，并使用逗号隔开即可。
- (int, int)

常用函数
```
len(tuple)——计算元组元素个数。
max(tuple)——返回元组中元素最大值。
min(tuple)——返回元组中元素最小值。
tuple(iterable)——将可迭代系列转换为元组。
```

### Set（集合）
- 集合（set）是一个无序的不重复元素序列。可以使用大括号 { } 或者 set() 函数创建集合
- 创建一个空集合必须用 set() 而不是 { }，因为 { } 是用来创建一个空字典

常用函数
```
len(s)——计算集合元素个数。
s.add(x)——给集合添加元素。
s.update(x)——给集合添加元素。
s.remove(x)——移除指定元素。
s.union(s2)——返回两个集合的并集。
s.clear()——清空集合。
x in s——判断元素是否在集合中存在
```

### if 语句
- 通过一条或多条语句的执行结果（True 或者 False）来决定执行的代码块。

```
if 表达式1:
    语句
    if 表达式2:
        语句
    elif 表达式3:
        语句
    else:
        语句
elif 表达式4:
    语句
else:
    语句
```

### match..case 语句
- 是 Python 3.10 增加了 match...case 的条件判断，不需要再使用一连串的 if-else 来判断了。

```
match subject:
    case <pattern_1>:
        <action_1>
    case <pattern_2>:
        <action_2>
    case <pattern_3>:
        <action_3>
    case _:
        <action_wildcard>
```

### for 语句

```
for <variable> in <sequence>:
    <循环主体>
else:
    <循环结束后执行的代码>
```

### while 语句

```
while <expr>:
    <statement(s)>
else:
    <additional_statement(s)>
```

#### break 语句可以跳出 for 和 while 的循环体
#### continue 语句可以跳过当前循环，然后继续进行下一轮循环

### 函数
- 使用 def 关键字
- 有参数默认值
- 有可变参数(参数前加 * 是以元组的形式传递)(参数前加 ** 是以字典的形式传递)
- 有多值返回，是以元组的形式

```
def get_name():
    pass
```

### 异常
- 手动抛出异常 raise

```
try:
    执行代码
except:
    发生异常时执行代码
else:
    没有异常时执行代码
finally:
    不管有没有异常都会执行代码
```

### 模块
- 导入模块
  - import 模块名
  - import 模块名 as 别名
  - from   模块名 import 函数名

### 类

```
#!/usr/bin/python3
 
class MyClass:
    i = 123456
    def f(self):
        return 'hello world'

# 实例化类
x = MyClass()
 
# 访问类的属性和方法
print("MyClass 类的属性 i 为：", x.i)
print("MyClass 类的方法 f 输出为：", x.f())
```








# String(字符串)：str
# 在 Python3 中，所有的 字符串 都是 Unicode 字符串

def string_run():
    a = '字符串'
    b = 100
    c = True
    d = f'{a},{b},{c}'  # python 3.6 之后版本添加
    e = '%s,%d,%s' % (a, b, c)
    print(d)  # 结果：字符串,100,True
    print(e)  # 结果：字符串,100,True
    
    print("======================================")

    # 字符串转成数字
    str_int = '123'
    if str_int.isnumeric():  # 是否 只包含数字字符
        print("转成数字:%d" % int(str_int))

    # 分隔 字符串，默认为空格
    str_split = '1 2    3\n4\n\r5'.split()
    print(str_split)  # 结果：['1', '2', '3', '4', '5']
    str_split = '6,7,8,9'.split(",")
    print(str_split)  # 结果：['6', '7', '8', '9']

    # 拼接 字符串
    str_join = ','.join(['A', 'B', 'C', 'D'])
    print(str_join)  # 结果：A,B,C,D

    # lstrip() 去除字符串左边的空格或指定字符
    # rstrip() 去除字符串末尾的空格或指定字符
    # 去除字符串两侧的空白字符 或 指定字符
    str_strip = ' EFG   \n\n\r\t'.strip()
    print(f'[{str_strip}]')  # 结果：[EFG]

    # 转换字符串中的小写字母为大写
    str_upper = "ab字母为大写cdEF".upper()
    print(str_upper)  # 结果：AB字母为大写CDEF

    # 转换字符串中所有大写字母为小写
    str_lower = 'AB字母为小写CDef'.lower()
    print(str_lower)  # 结果：ab字母为小写cdef

    # 在字符串出现的次数
    str_count = '出现a的a次数123出现a的a次数123'.count('次数')
    print(str_count)

    # 替换字符串
    str_replace = 'aaa替换aaa字符串aaa'.replace('a', '')
    print(str_replace)  # 结果：替换字符串

    # 判断字符串是否以 xxx 开头
    str_startswith = '开头abc开头123'.startswith("开头")
    print(str_startswith)  # 结果：True

    # 判断字符串是否以 xxx 结尾
    str_endswith = '以abc结尾123结尾'.endswith("结尾")
    print(str_endswith)  # 结果：True

    # 查找到第一次出现的位置
    str_find1 = '判断字符串是否包a含'.find('包含')  # 返回出现位置，如果没有找到，就返回 -1
    print(str_find1)  # 结果：-1
    
    # 判断字符串是否包含 xxx
    str_find2 = '包含' in '判断字符串是否包含'  # 返回 bool
    print(str_find2)  # 结果：True

    # 截取字符串
    str_substr = 'xxx截取字符串xxx'
    print(str_substr[3:8])  # 结果：截取字符串

    pass


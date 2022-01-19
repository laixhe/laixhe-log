# list(列表)、dict(字典)、set(集合)、tup(元组)

def list_dict_run():
    # 数据项不需要具有相同的类型(在项目中最好使用相同类型)

    # list(列表)
    # 索引从 0 开始
    lists = ['a', 'b', 'c']
    print(lists)  # 结果： ['a', 'b', 'c']
    lists.append('d')  # 追加元素
    print(lists)  # 结果： ['a', 'b', 'c', 'd']
    del lists[1]  # 删除元素
    print(lists)  # 结果： ['a', 'c', 'd']

    print("dict ------------------------")

    # dict(字典)以 键值对 key:value
    dicts = {'a': 1, 'b': 2, 'c': 3}
    print(dicts)  # 结果： {'a': 1, 'b': 2, 'c': 3}
    dicts['d'] = 4  # 不存在元素就新增元素，存在则修改
    print(dicts)  # 结果： {'a': 1, 'b': 2, 'c': 3, 'd': 4}
    del dicts['b']  # 删除元素，注意删除一个没有的元素会报错
    print(dicts)  # 结果： {'a': 1, 'c': 3, 'd': 4}

    print("set ------------------------")

    # set(集合)是一个无序的不重复元素序列
    sets = {'a', 'b', 'a', 'c'}
    print(sets)  # 结果： {'b', 'c', 'a'} 注意显示每次结果顺序都不同
    sets.add('d')  # 追加元素
    print(sets)  # 结果： {'d', 'c', 'a', 'b'}
    sets.remove('b')  # 移除元素，如果元素不存在，则会发生错误
    sets.discard('b')  # 移除元素，如果元素不存在，不会发生错误
    print(sets)  # 结果： {'c', 'd', 'a'}

    print("tup ------------------------")

    # tup(元组)的元素不能新增和修改
    # 索引从 0 开始
    tups = ('1', 2)
    print(tups, tups[1])  # 结果：('1', 2) 2

    print("tup ------------------------")

    pass

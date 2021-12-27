// List(列表)、Map(字典)、Set(集合)

void main() {
  // List(列表)
  // 可以不同的类型 var list = ['a', false, 'abc'];
  var list = <String>['a', 'b'];
  list.add('c');    // 追加元素
  list.remove('a'); // 移除元素，删除成功返回 true，失败返回 false
  print(list);      // 结果： [ b, c]

  // Map(字典)
  var map = <String, int>{'a':1, 'b': 2};
  map['c'] = 3;    // 不存在元素就新增元素，存在则修改
  map.remove('b'); // 移除元素，删除成功返回 其值，失败返回 null
  print(map);      // 结果： {a: 1, c: 3}

  // Set(集合)是一个元素唯一的无序队列
  var setT = <int>{3, 2, 3};
  setT.add(4);
  print(setT); // 结果： {3, 2, 4}
}
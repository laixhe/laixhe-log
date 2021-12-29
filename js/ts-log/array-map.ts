// Array(数组)、Map(字典)、Set(集合)、tuple(元组)

// 第一种定义方法：let 数组名：类型[] = []      (如：let arr: number[] = [1, 2])
// 第二种定义方法：let 数组名：Array[类型] = [] (如：let arr: Array<number> = [1, 2])

// Array(数组)的索引从 0 开始
// let arr: Array<string> = ["a", "b"];
let arr: string[] = ["a", "b"];
arr.push('c');    // 追加元素
console.log(arr); // 结果： [ 'a', 'b', 'c' ]
arr.shift();      // 删除并返回数组的第一个元素
arr.pop();        // 删除并返回数组最后一个元素
console.log(arr); // 结果： [ 'b' ]

console.log("=================================");

// Map(字典) 是 ES6 中引入的一种新的数据结构
let map: Map<string, number> = new Map([['a', 1], ['b', 2]]);
map.set('c', 3);  // 不存在元素就新增元素，存在则修改
console.log(map); // 结果： { 'a' => 1, 'b' => 2, 'c' => 3 }
map.delete('b');  // 删除 Map 中的元素，删除成功返回 true，失败返回 false

console.log("=================================");

// Set(集合)是一个不重复元素序列
let setT: Set<string> = new Set(['a', 'b', 'a']);
setT.add('c'); // 添加元素
console.log(setT); // 结果： { 'a', 'b', 'c' }

console.log("=================================");

// tuple(元组)，存储的元素数据类型不同（可进行增删改），索引从 0 开始
// 只能按类型的优先顺序输入内容，否则报错
let tuple: [number, string] = [1, "a"];
console.log(`tuple=${tuple}`);
console.log("tuple[1]=", tuple[1]);

#### 数组类型
> 原生数组

- ByteArray
- ShortArray
- IntArray
- LongArray
- FloatArray
- DoubleArray
- BooleanArray
- CharArray


```
// 创建数组
val arr1 = intArrayOf(1, 2, 3)
println(arr1.contentToString()) // 结果： [1, 2, 3]
// 创建一个指定大小且无素都为0的数组
val arr2 = IntArray(3){0}
println(arr2.contentToString()) // 结果： [0, 0, 0]
val arr3 = Array<Int>(3){0}
println(arr3.contentToString()) // 结果： [0, 0, 0]
val arrStr = Array<String>(3){""}
println(arrStr.contentToString()) // 结果： [, , ]

// 遍历数组
// 方式一
for (item in  arr1){
  println(item)
}
// 方式二，通过索引
for (index in  arr1.indices){
  println(arr1[index])
}
// 方式三，函数式
arr1.forEach { item ->
  println(item)
}

// 元素是否在数组内
if ( 1 in arr1){
  println("1确实在")
}
```

#### 集合类型



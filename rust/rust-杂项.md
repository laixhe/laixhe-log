### 常用
```
# size_of 获取某个类型的大小（以字节为单位）
# std::mem::size_of
let len1 = size_of::<char>(); 

# size_of_val 获取数据的实际大小（以字节为单位）
# std::mem::size_of_val
let len2 = mem::size_of_val(&len1);

```

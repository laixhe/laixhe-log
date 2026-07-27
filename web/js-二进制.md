
### JavaScript二进制

##### JavaScript 中如何存储和操作二进制数据
> ArrayBuffer 对象用来表示通用的、固定长度的原始二进制数据缓冲区

> 简单介绍下ArrayBuffer相关操作
>
> 示例通过创建一个长度为8Byte的二进制数据缓冲区
```
const buffer = new ArrayBuffer(8);
buffer.byteLength; // 结果为8
```

##### DataView
> DataView 视图是一个可以从 ArrayBuffer 对象中读写多种数值类型的底层接口，在读写时不用考虑平台字节序问题


##### 字节序
> 在现有的计算机体系中，有两种字节序

- 大端字节序：高位在前，低位在后。符合人类阅读习惯   0x  1 23 45 67
- 小端字节序：低位在前，高位在后。符合计算机读取习惯 0x 67 45 23 01

```
为什么会有小端字节序？

答案是，计算机电路先处理低位字节，效率比较高，因为计算都是从低位开始的。所以，计算机的内部处理都是小端字节序。

但是，人类还是习惯读写大端字节序。所以，除了计算机的内部处理，其他的场合几乎都是大端字节序，比如网络传输和文件储存
```

##### JavaScript 数字 与 二进制 换转
> 以Short类型和Int类型为例

```
let buffer = new ArrayBuffer(6); // 初始化6个Byte的二进制数据缓冲区
let dataView = new DataView(buffer);

dataView.setInt16(0, 3);  // 从第 0 个Byte位置开始，放置一个数字为 3  的Short类型数据(占2 Byte)
dataView.setInt32(2, 15); // 从第 2 个Byte位置开始，放置一个数字为 15 的Short类型数据(占4 Byte)

// 数据读取过程

let shortNumber = dataView.getInt16(0);
let intNumber = dataView.getInt32(2);

```

##### JavaScript String 与 二进制 换转
> UTF-16 是 JavaScript 中的 string 编码类型
>
> 后端常用的 UTF-8

```
new TextEncode().encode('data');             // 编码 返回 Unit8Array
new TextDecoder('utf-8').decode(Unit8Array); // 解码 返回 String

```


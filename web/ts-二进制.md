# TypeScript 二进制

## 如何存储和操作二进制数据

> `ArrayBuffer` 对象用来表示通用的、固定长度的原始二进制数据缓冲区。

下面通过一个长度为 8 Byte 的缓冲区示例简单介绍 `ArrayBuffer` 相关操作：

```typescript
const buffer: ArrayBuffer = new ArrayBuffer(8);
buffer.byteLength; // 结果为 8
```

## DataView

> `DataView` 视图是一个可以从 `ArrayBuffer` 对象中读写多种数值类型的底层接口，在读写时不用考虑平台字节序问题。

## 字节序

在现有的计算机体系中，有两种字节序：

- **大端字节序**：高位在前，低位在后，符合人类阅读习惯，例如 `0x 01 23 45 67`
- **小端字节序**：低位在前，高位在后，符合计算机读取习惯，例如 `0x 67 45 23 01`

```text
为什么会有小端字节序？

计算机电路先处理低位字节，效率比较高，因为计算都是从低位开始的，所以计算机内部处理都是小端字节序。

但人类习惯读写大端字节序，所以除了计算机内部处理，其他场合几乎都是大端字节序，比如网络传输和文件存储。
```

## 数字与二进制转换

以 Short 类型和 Int 类型为例：

```typescript
const buffer: ArrayBuffer = new ArrayBuffer(6); // 初始化 6 个 Byte 的二进制数据缓冲区
const dataView: DataView = new DataView(buffer);

dataView.setInt16(0, 3);  // 从第 0 个 Byte 位置开始，放置一个数字为 3 的 Short 类型数据（占 2 Byte）
dataView.setInt32(2, 15); // 从第 2 个 Byte 位置开始，放置一个数字为 15 的 Int 类型数据（占 4 Byte）

// 数据读取过程
const shortNumber: number = dataView.getInt16(0);
const intNumber: number = dataView.getInt32(2);
```

## String 与二进制转换

> JavaScript 中的 `string` 使用 UTF-16 编码，而后端常用 UTF-8。

```typescript
const encoded: Uint8Array = new TextEncoder().encode('data'); // 编码，返回 Uint8Array
const decoded: string = new TextDecoder('utf-8').decode(encoded); // 解码，返回 String
```

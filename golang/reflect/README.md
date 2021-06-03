
### 反射基本操作
```
反射 reflection

反射可大大提高程序的灵活性，使得 interface{} 有更大的发挥余地
反射使用 TypeOf 和 ValueOf 函数从接口中获取目标对象信息
反射会将匿名字段作为独立字段（匿名字段本质）
想要利用反射修改对象状态，前提是 interface.data 是 settable，

即 pointer-interface
- 通过反射可以“动态”调用方法
```
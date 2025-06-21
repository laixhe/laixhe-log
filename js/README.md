#### 安装环境
```
npm i -g typescript
npm install -g ts-node
tsc -v

tsc --noImplicitAny main.ts
```

- `注意：TypeScript 和 JavaScript 没有整数类型`

```
any        任意类型
number     数字类型(整数、浮点)( let age: number = 18 )
string     字符串类型( let name: string = "laixhe" )
boolean    布尔类型( let is: boolean = true )
*          数组类型( let arr: Array<number> = [1, 2] OR let arr: number[] = [1, 2] )
*          元组类型( let x: [string, number] )
*          Map类型( let m: Map<string, number> = new Map() OR let m = new Map<string, number>() )
*          Set类型( let s:Set<number> = new Set() OR let s = new Set<number>() )
enum       枚举( enum Color {Red, Green, Blue} )
interface  接口( interface interface_name {} )
class      类( class class_name {} )
namespace  命名空间( namespace namespace_name {} )
*          模块( export 导出 import 导入 )
```

#### 第一个例子
```
let hello : string = "Hello";
let world : string = "World";
let s1 : string = `${hello} ${world}!`;
let s2 : string = hello + " " + world + "!";
console.log(s1);
console.log(s2);
```

#### 函数
```
function getCount(num1: number, num2: number): number {
    return num1 + num2;
}
```

// 一切对象(未初化)的默认值都是 undefined (最好给于 初始值)，且没有整数类型，只有数字类型
// 可根据值自行判定变量类型

let world = 'World';
console.log(`Hello, ${world}`);

let a: number; // 未初始化的变量值为： undefined
let b: string = '中文'; 
let c = true; // 推断为 boolean 类型
let d = 1.1;  // 推断为 number 类型
// 结果： undefined 中文 true 1.1
console.log(a, b, c, d);

let arr: number[];
// 结果： undefined
console.log(arr);

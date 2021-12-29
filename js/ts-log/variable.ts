// 一切对象(未初化)的默认值都是 undefined (最好给于 初始值)，且没有整数类型，只有数字类型
// 可根据值自行判定变量类型

let world = 'World';
console.log(`Hello, ${world}`);
// 结果： Hello, World

// 定义变量时最好给于 初始值
// 未初始化的变量值为： undefined
// 在新的版本中 使用未初始化的变量会报错
let a: number = 1; 
let b: string = '中文'; 
let c = true; // 推断为 boolean 类型
let d = 1.1;  // 推断为 number 类型
// 结果： a=1, b=中文, c=true, d=1.1
console.log(`a=${a}, b=${b}, c=${c}, d=${d}`);

// 在老版本可以使用的，新版本报错
// let arr: number[];
// 结果： undefined
// console.log(arr); // 在 新版本 中不能使用未初始化的变量

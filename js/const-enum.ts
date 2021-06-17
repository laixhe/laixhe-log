// 由关键字 ```const``` 声明常量
// 由关键字 ```enum``` 枚举类型有数字枚举、字符串枚举，可以手动指定原始值

//

// 枚举
// 默认情况下，从 0 开始为元素编号
// 可以手动指定原始值
enum Color {
    read,
    green,
    blue
}
console.log(Color.blue); // 结果： 2

// 常量
const uid: number = 10;
//uid = 11; // error

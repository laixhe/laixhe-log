// 由关键字 ```const``` 声明常量
// 由关键字 ```enum``` 枚举类型有数字枚举、字符串枚举，可以手动指定原始值

// 枚举
// 默认情况下，从 0 开始为元素编号
// 可以手动指定原始值
enum Color {
    read,
    green,
    blue
}
console.log("Color.blue=", Color.blue); // 结果： Color.blue= 2
console.log("Color[Color.blue]=", Color[Color.blue]); // 结果： Color[Color.blue]= blue

enum School {
    student = '学生',
    teacher = '教师',
    parent = '家长'
}
console.log(`School.teacher=${School.teacher}`); // 结果： School.teacher=教师

// 常量
const COLOR_ID: number = 10;
//COLOR_ID = 11; // error
console.log("COLOR_ID=", COLOR_ID); // 结果： COLOR_ID= 10

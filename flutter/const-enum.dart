// 由关键字 ```const``` 声明常量
// 由关键字 ```enum``` 枚举类型且不能手动指定原始值

//

// 枚举
// 从 0 开始为元素编号
// 不能手动指定原始值
enum Color {
  read,
  green,
  blue
}

// const：值不变，一开始就得赋值
const int uid = 10;
// final：可以开始不赋值，只能赋一次
final String name = 'laixhe';

void main() {
  print('${Color.blue}, ${Color.blue.index}'); // 结果： Color.blue, 2
}
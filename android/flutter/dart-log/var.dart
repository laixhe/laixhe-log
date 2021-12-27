// 一切对象(未初化)的默认值都是 null (最好给于 初始值)
// 可根据值自行判定变量类型

void main() {
  var world = 'World';
  print('Hello, $world');
  
  int? a;       // 未初始化的变量值为： null
  String b = '中文';
  var c = true; // 推断为 bool 类型
  var d = 1.1;  // 推断为 double 类型
  var e = 2;    // 推断为 int 类型
  // 结果： [null, 中文, true, 1.1, 2]
  print([a,b,c,d,e]);

  List<int>? arr;
  // 结果： null
  print(arr);
}

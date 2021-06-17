// 无关键字
// 有可选命名参数
// 有默认参数值

//

// 无参数，无返回值
void Void(){
  print('void');
}
// 有可选命名参数和默认值，有返回值
int Test(int a, {bool b=false, String? s}){
  print([a, b, s]);
  return a;
}

void main() {
  Void();          // 结果： void
  var t = Test(1); // 结果： [1, false, null]
  print(t);        // 结果： 1
}
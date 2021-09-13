// 面向对象的三个基本特征：封装(Encapsulation)、继承(Inheritance)、多态(Polymorphism)

// 由关键字```class```声明，没有 public、protected、private 语法关键字，_ 下划线直接代表 private 
// 默认情况下，成员是公共的，且没有方法重载，也没有 interface 接口修饰，只有抽象类 abstract

//

// 定义抽象类
abstract class BaseInterface {
  void Show();
}

// 定义类并实现了 BaseInterface 抽象类
class Base extends BaseInterface{
  int id = 0; // 最好给于默认值否则为 null
  String name = '';

  // 构造函数
  Base(this.id, this.name);

  @override
  void Show() {
    print('base show id=$id, name=$name');
  }
}

// 继承 Base
class TestBase extends Base {
  // 静态属性
  static int age = 0;

  TestBase(int id, String name) : super(id, name);

  @override
  void Show() {
    print('base show id=$id, name=$name');
  }

  // 静态方法
  static void Test(){
    print(age);
    age++;
  }
}

// 利用接口实现多态
void Show(BaseInterface b) {
  b.Show();
}

void main() {
  var base = Base(18, 'laixhe');
  base.Show(); // 结果：base show id=18, name=laixhe
  var testBase = TestBase(19, 'lai');
  base.Show(); // 结果：base show id=18, name=laixhe

  Show(base);     // 结果：base show id=18, name=laixhe
  Show(testBase); // 结果：base show id=19, name=lai
  
  print(TestBase.age); // 结果： 0
  TestBase.Test();     // 结果： 0
  print(TestBase.age); // 结果： 1
}

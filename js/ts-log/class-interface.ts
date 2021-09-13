// 面向对象的三个基本特征：封装(Encapsulation)、继承(Inheritance)、多态(Polymorphism)

// 由关键字```class```声明，有两个访问修饰符 public 和 private 默认情况下，成员是 public 的

//

// 定义接口
interface BaseInterface {
    id: number;
    name: string;
    show(): void;
}

// 定义类并实现了 BaseInterface 接口
class Base implements BaseInterface{
    id:   number = 0; // 字段最好给于初始默认值否则为 undefined
    name: string = '';
    // 构造函数
    constructor(id: number, name: string){
        this.id = id;
        this.name = name;
    }
    // 成员方法并实现 BaseInterface 接口的方法
    show(): void {
        console.log(`base show id=${this.id}, name=${this.name}`);
    }
}

// 继承 Base
class TestBase extends Base {
    // 静态属性
    static age: number = 18;
    // 静态方法
    static Test(){
        console.log(this.age);
        this.age++;
    }
}

// 利用接口实现多态
function Show(b: BaseInterface): void {
    b.show();
}

// 实例化对象
let base = new Base(18, 'laixhe');
base.show();               // 结果： base show id=18, name=laixhe

// 直接调用 静态
console.log(TestBase.age); // 结果： 18
TestBase.Test();           // 结果： 18
let testBase = new TestBase(16, 'lai');
testBase.show();           // 结果： base show id=16, name=lai
TestBase.Test();           // 结果： 19

Show(base);     // 结果： base show id=18, name=laixhe
Show(testBase); // 结果： base show id=16, name=lai

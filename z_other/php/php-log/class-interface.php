<?php

// 面向对象的三个基本特征：封装(Encapsulation)、继承(Inheritance)、多态(Polymorphism)

// 由关键字```class```声明

// 定义接口
interface BaseInterface{
    public function Show():void;
}

// 定义类并实现了 BaseInterface 接口
class Base implements BaseInterface {
    public int $id = 0;
    public string $name = '';

    // 构造函数
    public function __construct(int $id, string $name){
        $this->id   = $id;
        $this->name = $name;
    }

    // 析构函数
    //public function __destruct(){}

    // 成员方法并实现 BaseInterface 接口的方法
    public function Show(): void {
        echo 'base show id=', $this->id, 'name=', $this->name;
    }
}

// 利用接口实现多态
function Show(BaseInterface $b): void {
    $b->Show();
}

// 定义类并继承 Base
class TestBase extends Base {
    // 静态属性
    public static int $age = 18;
    // 静态方法
    public static function Test(){
        echo self::$age;
        self::$age++;
    }

    public function Show(): void {
        echo 'test base show';
    }
}

$base = new Base(18, 'laixhe');
$base->Show();     // 结果： base show id=18name=laixhe
echo "\n";

$testBase = new TestBase(20, 'lai');
$testBase->Show(); // 结果： test base show
echo "\n";
TestBase::Test();  // 结果： 18
echo "\n";
TestBase::Test();  // 结果： 19

echo "\n";
Show($base);       // 结果： base show id=18name=laixhe
echo "\n";
Show($testBase);   // 结果： test base show

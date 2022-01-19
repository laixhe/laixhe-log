<?php
// PHP本身没有类型声明，会根据变量的值，自动把变量转换为正确的数据类型 
// (在执行间期可以改变为其它类型) 
// 尽量使用 php7.4.x 或 php8.x 以上版本 (使用强类型声明)
//

$world = 'World';
echo 'Hello, '.$world;
echo "\n";
echo "Hello, $world";

$a = 123;   // 系统推断为 int 类型
$b = '中文'; // 系统推断为 string 类型
$c = true;  // 系统推断为 bool 类型
$d = 1.1;   // 系统推断为 float 类型
// 上面虽然系统推断出类型，当期间可以随意更改为其它类型
// 结果为：
//array(4) {
//  [0]=>
//  int(123)
//  [1]=>
//  string(6) "中文"
//  [2]=>
//  bool(true)
//  [3]=>
//  float(1.1)
//}
var_dump([$a ,$b, $c, $d]);

// 动态改变类类型
$e = '中文';
var_dump($e); // 结果为： string(6) "中文"
$e = 100;
var_dump($e); // 结果为： int(100)

// 强类型 class (可使用 强类型 来作为数据的承载，防止数据类型被修改)，并给相应的默认值防止没有初始化
class User {
    public int    $uid = 0;
    public string $name = '';
    public bool   $isAuto = false;
    public function __construct(int $uid, string $name, $isAuto){
        $this->uid    = $uid;
        $this->name   = $name;
        $this->isAuto = $isAuto;
    }
}
// 结果为：
// object(User)#1 (3) {
//  ["uid"]=>
//  int(123)
//  ["name"]=>
//  string(6) "中文"
//  ["isAuto"]=>
//  bool(false)
//}
$user = new User(uid: 123, name: "中文", isAuto: false);
var_dump($user);

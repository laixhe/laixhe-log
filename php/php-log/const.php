<?php
// 由关键字```const```声明常量，没有枚举

//

// 定义常量
define('UID', 10);
const NAME = 'laixhe';

echo UID,', ', NAME, "\n"; // 结果： 10, laixhe

// 在 php 中没有枚举，使用 class 配合 const 来实现
class Color{
    const read = 0;
    const green = 1;
    const blue = 2;
}
echo Color::blue; // 结果： 2

<?php
// 由关键字```function```声明
// 有默认参数值
// 有可变长参数(不定长参数)

//

// 无参数，无返回值
function Void(): void {
    echo "void\n";
}

// 可变长参数(不定长参数)
function Names(string ...$s){
    var_dump($s);
}

// 有参数有默认参数值，有返回值
function Test(int $a, bool $b=false, string $s='str...'): int{
    echo $a, ',',$b, ',', $s, "\n";
    return $a;
}

Void();               // 结果： void
Names('a', 'b', 'c'); // 结果：array(3) {[0]=>string(1) "a" [1]=>string(1) "b" [2]=>string(1) "c"}
$t = Test(1);        // 结果： 1,,str...
echo "$t\n";         // 结果： 1

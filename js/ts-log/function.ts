// 由关键字```function```声明
// 有可变长参数
// 有可选参数
// 有默认参数值

//

// 无参数，无返回值
function Void(): void {
    console.log('void');
}
// 可变长参数(不定长参数)
function Names(...s: string[]){
    console.log(s);
}
// 有参数有默认参数值，有返回值
function Test(a: number, b: boolean=false, s?: string): number {
    console.log(a, b, s);
    return a;
}
// 使用元组形式返回多值不同类型的值
function Tuple(): [number, string] {
    return [10, 'Z'];
}

Void();                     // 结果： void
Names('a', 'b', 'c');       // 结果： [ 'a', 'b', 'c' ]
let t = Test(1);            // 结果： 1 false undefined
console.log(t);             // 结果： 1 
let [uid, uname] = Tuple(); // 解构元组
console.log(uid, uname);    // 结果： 10 Z

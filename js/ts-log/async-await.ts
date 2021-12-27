// 核心
// 1. async 定义的函数内部会默认返回一个 promise 对象
// 2. await 异步等待(相当于 promise 的 then)，只能放在 async 函数里
// 3. try...catch 可捕获异常，代替了 promise 的 catch 与 catch

async function asyncFn() : Promise<string>{
    // return '异步...'; // 相当于 return Promise.resolve('异步...')
    return Promise.reject('异步错误...');
}

asyncFn().then(result => {
    console.log("成功："+result);
}).catch(result => {
    console.log("失败："+result);
});

console.log('先执行...');


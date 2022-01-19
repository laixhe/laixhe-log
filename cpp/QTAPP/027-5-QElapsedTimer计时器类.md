#### 用例
```
// 计时器 - 用于计算某段代码的执行耗时
QElapsedTimer etime;
// 开始计时
etime.start();

//相关操作

// 得到耗时操作所花费的具体时间，以毫秒计算
int milsec = etime.elapsed();

qDebug() << "执行耗时:" << milsec << "毫秒";
```

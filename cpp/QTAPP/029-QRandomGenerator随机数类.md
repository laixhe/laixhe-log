#### 老方法
> 利用 qrand 和 qsrand 生成随机数
>
> 位于 QtGlobal 中
> 
```
qsrand(QTime::currentTime().msec());	// 设置种子，该种子作为qrand生成随机数的起始值，RAND_MAX为32767，即随机数在种子值到32767之间
qrand()%10;
```

#### 新方法
> 利用 QRandomGenerator 类
>
> Qt5.10之后新增该类
>

```
qDebug()<<QRandomGenerator::global()->bounded(10);		//生成一个0和10之间的整数
qDebug()<<QRandomGenerator::global()->bounded(10.123);	//生成一个0和10.123之间的浮点数
qDebug()<<QRandomGenerator::global()->bounded(10, 15);	//生成一个10和15之间的整数
```

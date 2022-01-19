> QTime 类可以封装时间信息也可以通过这个类得到时间相关的信息，包括: 时 , 分 , 秒 , 毫秒

```
// 构造函数
QTime::QTime();

h 		    ==> 取值范围： 0 ~ 23
m and s 	==> 取值范围： 0 ~ 59
ms 		    ==> 取值范围： 0 ~ 999

QTime::QTime(int h, int m, int s = 0, int ms = 0);

// 公共成员函数
// Returns true if the set time is valid; otherwise returns false.
bool QTime::setHMS(int h, int m, int s, int ms = 0);
QTime QTime::addSecs(int s) const;
QTime QTime::addMSecs(int ms) const;

// 示例代码
QTime n(14, 0, 0);                // n == 14:00:00
QTime t;
t = n.addSecs(70);                // t == 14:01:10
t = n.addSecs(-70);               // t == 13:58:50
t = n.addSecs(10 * 60 * 60 + 5);  // t == 00:00:05
t = n.addSecs(-15 * 60 * 60);     // t == 23:00:00

// 从时间对象中取出 时/分/秒/毫秒
// Returns the hour part (0 to 23) of the time. Returns -1 if the time is invalid.
int QTime::hour() const;
// Returns the minute part (0 to 59) of the time. Returns -1 if the time is invalid.
int QTime::minute() const;
// Returns the second part (0 to 59) of the time. Returns -1 if the time is invalid.
int QTime::second() const;
// Returns the millisecond part (0 to 999) of the time. Returns -1 if the time is invalid.
int QTime::msec() const;


// 时间格式化

-- 时 --
h	==>	The hour without a leading zero (0 to 23 or 1 to 12 if AM/PM display)
hh	==>	The hour with a leading zero (00 to 23 or 01 to 12 if AM/PM display)
H	==>	The hour without a leading zero (0 to 23, even with AM/PM display)
HH	==>	The hour with a leading zero (00 to 23, even with AM/PM display)
-- 分 --
m	==>	The minute without a leading zero (0 to 59)
mm	==>	The minute with a leading zero (00 to 59)
-- 秒 --
s	==>	The whole second, without any leading zero (0 to 59)
ss	==>	The whole second, with a leading zero where applicable (00 to 59)
-- 毫秒 --
zzz	==>	The fractional part of the second, to millisecond precision, 
        including trailing zeroes where applicable (000 to 999).
-- 上午或者下午
AP or A	==>	使用AM/PM(大写) 描述上下午, 中文系统显示汉字
ap or a	==>	使用am/pm(小写) 描述上下午, 中文系统显示汉字

QString QTime::toString(const QString &format) const;

// 阶段性计时
// 过时的API函数
// 开始计时
void QTime::start();
// 计时结束
int QTime::elapsed() const;
// 重新计时
int QTime::restart();

// 推荐使用的API函数
// QElapsedTimer 类
void QElapsedTimer::start();
qint64 QElapsedTimer::restart();
qint64 QElapsedTimer::elapsed() const;


// 操作符重载 ==> 时间比较
bool QTime::operator!=(const QTime &t) const;
bool QTime::operator<(const QTime &t) const;
bool QTime::operator<=(const QTime &t) const;
bool QTime::operator==(const QTime &t) const;
bool QTime::operator>(const QTime &t) const;
bool QTime::operator>=(const QTime &t) const;

// 静态函数 -> 得到当前时间
[static] QTime QTime::currentTime();
```

> QPoint 类封装了我们常用用到的坐标点 (x, y)

```
// 构造函数
// 构造一个坐标原点, 即(0, 0)
QPoint::QPoint();
// 参数为 x轴坐标, y轴坐标
QPoint::QPoint(int xpos, int ypos);

// 设置x轴坐标
void QPoint::setX(int x);
// 设置y轴坐标
void QPoint::setY(int y);

// 得到x轴坐标
int QPoint::x() const;
// 得到x轴坐标的引用
int &QPoint::rx();
// 得到y轴坐标
int QPoint::y() const;
// 得到y轴坐标的引用
int &QPoint::ry();

// 直接通过坐标对象进行算术运算: 加减乘除
QPoint &QPoint::operator*=(float factor);
QPoint &QPoint::operator*=(double factor);
QPoint &QPoint::operator*=(int factor);
QPoint &QPoint::operator+=(const QPoint &point);
QPoint &QPoint::operator-=(const QPoint &point);
QPoint &QPoint::operator/=(qreal divisor);
```
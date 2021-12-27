> 很多时候，需要几种不同的数据类型需要传递，如果用结构体，又不大方便，容器保存的也只是一种数据类型，而 QVariant 则可以统统搞定。
>
> QVariant 可以保存很多 Qt 的数据类型，包括 QBrush、QColor、QCursor、QDateTime、QFont、QKeySequence、 QPalette、QPen、QPixmap、QPoint、QRect、QRegion、QSize和QString，并且还有 C++ 基本类型，如 int、float 等...

#### 将标准类型转换为 QVariant 类型
```
// 使用设置函数也可以将支持的类型的数据设置到QVariant对象中
// 这里的 T 类型, 就是QVariant支持的类型
void QVariant::setValue(const T &value);
// 该函数行为和 setValue() 函数完全相同
[static] QVariant QVariant::fromValue(const T &value);

// 例子:
QVariant v = QVariant::fromValue(5);

int i = v.toInt();          // i is now 5
QString s = v.toString();   // s is now "5"
```

#### 判断 QVariant 中封装的实际数据类型
> 返回值 Type 的部分枚举定义，全部信息可以自行查阅 Qt 帮助文档
```
// 该函数的返回值是一个枚举类型, 可通过这个枚举判断出实际是什么类型的数据
Type QVariant::type() const;
```

#### 自定义类型
> 除了标准类型，我们自定义的类型也可以使用 QVariant 类进行封装，被QVariant存储的数据类型需要有一个默认的构造函数和一个拷贝构造函数
>
> 为了实现自定义类型，首先必须使用 Q_DECLARE_METATYPE() 宏
>

```
# 1. 在头文件中声明
// *.h
struct MyTest {
    int id;
    QString name;
};
// 自定义类型注册
Q_DECLARE_METATYPE(MyTest)

# 2. 在源文件中定义
MyTest t;
t.name = "laixhe";
t.num = 888;
// 值的封装
QVariant vt = QVariant::fromValue(t);

// 值的读取
if(vt.canConvert<MyTest>()){
    MyTest t = vt.value<MyTest>();
    qDebug() << "name: " << t.name << ", num: " << t.num;
}
```
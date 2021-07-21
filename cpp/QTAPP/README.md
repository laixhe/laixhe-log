### 模块
```
Qt Core                  Qt类库的核心，所有其它模块都依赖于此模块
Qt GUI                   设计 GUI 界面的基础类，包括 OpenGL
Qt Widgets               用于构建 GUI 界面的 C++ 图形组件类
Qt Multimedia            音频、视频、摄像头和广播功能的类
Qt Multimedia Widgets    实现多媒体功能的界面组件类
Qt Network               使网络编程更简单和轻便的类
Qt QML                   用于 QML 和 JavaScript 的类
Qt Quick                 用于构建具有定制用户界面的动态应用程序的声明框架
Qt Quick Controls        创建桌面样式用户界面，基于 Qt Quick 的用户界面控件
Qt Quick Dialogs         用于 Qt Quick 的系统对话框类
Qt Quick Layouts         用于 Qt Quick 2 界面元素的布局项
Qt SQL                   使用 SQL 用于数据库操作类
Qt Test                  用于应用程序和库进行单元测试的类
```

#### 常用类
```
QVariant  数据类型的联合类
QPoint    坐标类，封装了我们常用用到的坐标点 (x, y)
QLine     直线类，封装了两个坐标点 (两点确定一条直线)
QSize     尺寸类，用来形容长度和宽度
QRect     矩形类
QDate     日期类
QTime     时间类
QDateTime 日期和时间类，其实这个类就是 QDate 和 QTime 这两个类的结合体
QTimer    定时器类
QColor    颜色类
QBrush    画刷类，用于填充，如矩形、椭圆形或多边形等形状
QPen      画笔类，用于绘制轮廓线
QPixmap   纹理类(像素图)，用于绘图设备的图像显示，依赖于硬件
QImage 
QBitmap
QPicture  保存绘图的状态(二进制文件)
QGradient 渐变类
QPainter  绘图操作类
   
```

#### QWidget
> 所有窗口类的基类
>
> 可以内嵌到其它窗口的内部-无边框(需要给该窗口指定父窗口)
>
> 可以作为独立的窗口显示-有边框(不能给该窗口指定父窗口)
>
> 是 Qt 中所有控件的基类
>

#### QMainWindow
> 主窗口类
>
> 可以包含菜单栏、工具栏、状态栏
>
> 不能内嵌
>

#### QDialog
> 对话框窗口类
> 
> 模态(show)和非模态(exec)两种显示方式
> 
> 不能内嵌
> 

### QT 的坐标原点在窗口的左上角
> 子窗口的坐标原点在父窗口的左上角

```
.------------------X
|坐标
|  原点
|
|
|
Y
```

### QT 中的内存回收机制
> 在 Qt 中创建对象的时候会提供一个`Parent对象指针`(可以查看类的构造函数)
>
> QObject 是以对象树的形式组织起来的
> >
> > 当你创建一个 QObject 对象时，会看到 QObject 的构造函数接收一个 QObject 指针作为参数，这个参数就是 parent 也就是父对象指针。这相当于，在创建QObject对象时，可以提供一个其父对象，我们创建的这个QObject对象会自动添加到其父对象的children()列表。当父对象析构的时候，这个列表中的所有对象也会被析构。(注意，这里的父对象并不是继承意义上的父类！)
>
> Qt 中有内存回收机制, 但是不是所有被 new 出的对象被自动回收, 满足条件才可以回收，需要满足以下两个条件：
>
> > 1. 创建的对象必须是 QObject 类的子类(间接子类也可以)
> >
> > 2. 创建出的类对象，必须要指定其父对象是谁，一般情况下有两种操作方式：
>

```
# 方式1: 通过构造函数
# parent: 当前窗口的父对象, 找构造函数中的 parent 参数即可
QWidget::QWidget(QWidget *parent = Q_NULLPTR, Qt::WindowFlags f = Qt::WindowFlags());
QTimer::QTimer(QObject *parent = nullptr);

# 方式2: 通过 setParent() 方法
# 假设这个控件没有在构造的时候指定符对象, 可以调用QWidget的api指定父窗口对象
void QWidget::setParent(QWidget *parent);
void QObject::setParent(QObject *parent);
```
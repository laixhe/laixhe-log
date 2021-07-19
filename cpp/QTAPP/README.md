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
> QObject 是以对象树的形式组织起来
>
> 如果想要在 QT 中实现内存的自动回收，需要满足以下两个条件：
>
> > 1. 创建的对象必须是 QObject 类的子类(间接子类也可以)
> >
> > 2. 创建出的类对象，必须要指定其父对象是谁，一般情况下有两种操作方式：
>

```
# 方式1：通过构造函数
# parent 当前窗口的父对象，找构造函数中的 parent 参数即可
QWidget::QWidget(QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags())

# 方式2：通过 setParent() 方式
void QWidget::setParent(QWidget* parent)
```

#### 自定义信号槽
> 如果想要使用自定义的信号槽，首先要编写新的类并且要继承 QObject 类或者其子类，还要在定义类的头文件中加入 QObject 中的 Q_OBJECT 这个宏
>

##### 自定义信号 signals
> 信号是类的成员函数
> 
> 要使用 signals 关键字进行声明，使用方法与 public 类似
>
> 返回值是 void 类型
> 
> 信号函数只需要声明，不需要定义(没有函数体实现)
> 
> 发送信号函数前加关键字： emit
> 

###### 自定义槽 slots
> 槽函数就是信号的处理动作，自定义槽函数和自定义的普通函数一样
> 
> 返回值是 void 类型
> 
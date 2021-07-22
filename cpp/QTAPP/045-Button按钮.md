> 常用的按钮控件，包括: QAbstractButton, QPushButton, QToolButton, QRadioButton, QCheckBox

#### QAbstractButton 是所有按钮的基类

##### 标题和图标
```
// 参数text的内容显示到按钮上
void QAbstractButton::setText(const QString &text);
// 得到按钮上显示的文本内容, 函数的返回就是
QString QAbstractButton::text() const;

// 得到按钮设置的图标
QIcon icon() const;
// 给按钮设置图标
void setIcon(const QIcon &icon);

// 得到按钮图标大小
QSize iconSize() const
// 设置按钮图标的大小
[slot]void setIconSize(const QSize &size);
```

##### 按钮的 Check 属性
> 对应按钮来说，一般有三种常见状态，分别为: Normal, Hover, Pressed
> >
> > Normal: 普通状态，没有和鼠标做任何接触
> >
> > Hover: 悬停状态，鼠标位于按钮之上，但是并未按下
> >
> > Pressed: 按压状态，鼠标键在按钮上处于按下状态
>
> 默认情况下，鼠标在按钮上按下，按钮从 Normal 切换到 Pressed 状态，鼠标释放，按钮从 Pressed 恢复到 Normal 状态
>
> 当我们给按钮设置了 check 属性之后，情况就有所不同了， 在按钮上释放鼠标键， 按钮依然会处在 Pressed 状态，再次点击按钮，按钮才能恢复到 Normal 状态。具有 check 属性的按钮就相当于一个开关，每点击一次才能实现一次状态的切换
>

```
// 判断按钮是否设置了checkable属性, 如果设置了点击按钮, 按钮一直处于选中状态
// 默认这个属性是关闭的, not checkable
bool QAbstractButton::isCheckable() const;
// 设置按钮的checkable属性
// 参数为true: 点击按钮, 按钮被选中, 松开鼠标, 按钮不弹起
// 参数为false: 点击按钮, 按钮被选中, 松开鼠标, 按钮弹起
void QAbstractButton::setCheckable(bool);

    
// 判断按钮是不是被按下的选中状态
bool QAbstractButton::isChecked() const;
// 设置按钮的选中状态: true-选中, false-没选中
// 设置该属性前, 必须先进行 checkable属性的设置
void QAbstractButton::setChecked(bool);
```

##### 信号
> 这些信号都按钮被点击之后发射出来的，只是在细节上有细微的区别，其中最常用的是 clicked(), 通过鼠标的不同瞬间状态可以发射出 pressed() 和 released() 信号，如果鼠标设置了 check 属性，一般通过 toggled() 信号判断当前按钮是选中状态还是非选中状态。

```
/*
当按钮被激活时(即，当鼠标光标在按钮内时按下然后释放)，当键入快捷键时，或者当click()或animateClick()被调用时，这个信号被发出。值得注意的是，如果调用setDown()、setChecked()或toggle()，则不会触发此信号。
*/
[signal] void QAbstractButton::clicked(bool checked = false);
// 在按下按钮的时候发射这个信号
[signal] void QAbstractButton::pressed();
// 在释放这个按钮的时候发射直观信号
[signal] void QAbstractButton::released();
// 每当可检查按钮改变其状态时，就会发出此信号。checked在选中按钮时为true，在未选中按钮时为false。
[signal] void QAbstractButton::toggled(bool checked);
```

##### 槽函数
```
// 执行一个动画点击:按钮被立即按下，并在毫秒后释放(默认是100毫秒)。
[slot] void QAbstractButton::animateClick(int msec = 100);
// 执行一次按钮点击, 相当于使用鼠标点击了按钮
[slot] void QAbstractButton::click();

// 参考 1.2 中的函数介绍
[slot] void QAbstractButton::setChecked(bool);
// 设置按钮上图标大小
[slot]void setIconSize(const QSize &size);
// 切换可检查按钮的状态。 checked <==> unchecked
[slot] void QAbstractButton::toggle();
```

#### QPushButton
> 这个类是一个常用按钮类, 操作这个按钮使用的大部分函数都是从父类继承过来的, 它的父类是 QAbstractButton

```
# 构造函数

# 参数:
- icon: 按钮上显示的图标
- text: 按钮上显示的标题
- parent: 按钮的父对象, 可以不指定

QPushButton::QPushButton(const QIcon &icon, const QString &text, QWidget *parent = nullptr);
QPushButton::QPushButton(const QString &text, QWidget *parent = nullptr);
QPushButton::QPushButton(QWidget *parent = nullptr);

判断按钮是不是默认按钮
bool isDefault() const;
一般在对话框窗口中使用, 将按钮设置为默认按钮, 自动关联 Enter 键 
void setDefault(bool);

将弹出菜单菜单与此按钮关联起来。这将把按钮变成一个菜单按钮，
在某些样式中会在按钮文本的右边产生一个小三角形。
void QPushButton::setMenu(QMenu *menu);

显示(弹出)相关的弹出菜单。如果没有这样的菜单，这个函数什么也不做。
这个函数直到弹出菜单被用户关闭后才返回。
[slot] void QPushButton::showMenu();
```

#### QToolButton (增强版按钮)
> 使用方法和功能跟 QPushButton 基本一致, 只不过在和菜单进行管理这个方面,QToolButton 类可以设置弹出的菜单的属性,以及在显示图标的时候可以设置更多的样式, 可以理解为是一个增强版的 QPushButton

```
///////////////////////////// 构造函数 /////////////////////////////
QToolButton::QToolButton(QWidget *parent = nullptr);

/////////////////////////// 公共成员函数 ///////////////////////////
/*
    1. 将给定的菜单与此工具按钮相关联。
    2. 菜单将根据按钮的弹出模式显示。
    3. 菜单的所有权没有转移到“工具”按钮(不能建立父子关系)
*/
void QToolButton::setMenu(QMenu *menu);
// 返回关联的菜单，如果没有定义菜单，则返回nullptr。
QMenu *QToolButton::menu() const;

/*
弹出菜单的弹出模式是一个枚举类型: QToolButton::ToolButtonPopupMode, 取值如下:
    - QToolButton::DelayedPopup: 
        - 延时弹出, 按压工具按钮一段时间后才能弹出, 比如:浏览器的返回按钮
        - 长按按钮菜单弹出, 但是按钮的 clicked 信号不会被发射
    - QToolButton::MenuButtonPopup: 
        - 在这种模式下，工具按钮会显示一个特殊的箭头，表示有菜单。
	- 当按下按钮的箭头部分时，将显示菜单。按下按钮部分发射 clicked 信号
    - QToolButton::InstantPopup: 
        - 当按下工具按钮时，菜单立即显示出来。
        - 在这种模式下，按钮本身的动作不会被触发(不会发射clicked信号)
*/
// 设置弹出菜单的弹出方式
void setPopupMode(QToolButton::ToolButtonPopupMode mode);
// 获取弹出菜单的弹出方式
QToolButton::ToolButtonPopupMode popupMode() const;

/*
QToolButton可以帮助我们在按钮上绘制箭头图标, 是一个枚举类型, 取值如下: 
    - Qt::NoArrow: 没有箭头
    - Qt::UpArrow: 箭头向上
    - Qt::DownArrow: 箭头向下
    - Qt::LeftArrow: 箭头向左
    - Qt::RightArrow: 箭头向右
*/
// 显示一个箭头作为QToolButton的图标。默认情况下，这个属性被设置为Qt::NoArrow。
void setArrowType(Qt::ArrowType type);
// 获取工具按钮上显示的箭头图标样式
Qt::ArrowType arrowType() const;

///////////////////////////// 槽函数 /////////////////////////////
// 给按钮关联一个QAction对象, 主要目的是美化按钮
[slot] void QToolButton::setDefaultAction(QAction *action);
// 返回给按钮设置的QAction对象
QAction *QToolButton::defaultAction() const;

/*
图标的显示样式是一个枚举类型->Qt::ToolButtonStyle, 取值如下:
    - Qt::ToolButtonIconOnly: 只有图标, 不显示文本信息
    - Qt::ToolButtonTextOnly: 不显示图标, 只显示文本信息
    - Qt::ToolButtonTextBesideIcon: 文本信息在图标的后边显示
    - Qt::ToolButtonTextUnderIcon: 文本信息在图标的下边显示
    - Qt::ToolButtonFollowStyle: 跟随默认样式(只显示图标)
*/
// 设置的这个属性决定工具按钮是只显示一个图标、只显示文本，还是在图标旁边/下面显示文本。
[slot] void QToolButton::setToolButtonStyle(Qt::ToolButtonStyle style);
// 返回工具按钮设置的图标显示模式
Qt::ToolButtonStyle toolButtonStyle() const;


// 显示相关的弹出菜单。如果没有这样的菜单，这个函数将什么也不做。这个函数直到弹出菜单被用户关闭才会返回。
[slot] void QToolButton::showMenu();
```

#### QRadioButton (单选按钮)
> 是 Qt 提供的单选按钮，一般都是`以组`的方式来使用 (多个按钮中同时只能选中其中一个)

```
# 构造函数
# 参数:
- text: 按钮上显示的标题
- parent: 按钮的父对象

QRadioButton::QRadioButton(const QString &text, QWidget *parent = nullptr);
QRadioButton::QRadioButton(QWidget *parent = nullptr);
```

#### QCheckBox (复选框(多选)按钮)
> 是 Qt 中的复选框按钮，可以单独使用，也可以以组的方式使用 (同一组可以同时选中多个)

```
# 构造函数
# 参数:
- text: 按钮上显示的文本信息
- parent: 按钮的父对象

QCheckBox::QCheckBox(const QString &text, QWidget *parent = nullptr);
QCheckBox::QCheckBox(QWidget *parent = nullptr);

// 判断当前复选框是否为三态复选框, 默认情况下为两种状态: 未选中, 选中
bool isTristate() const;
// 设置当前复选框为三态复选框: 未选中, 选中, 半选中
void setTristate(bool y = true);

/*
参数 state, 枚举类型 Qt::CheckState:
    - Qt::Unchecked	      --> 当前复选框没有被选中
    - Qt::PartiallyChecked    --> 当前复选框处于半选中状态, 部分被选中(三态复选框)
    - Qt::Checked	      --> 当前复选框处于选中状态
*/
// 设置复选框按钮的状态
void QCheckBox::setCheckState(Qt::CheckState state);
// 获取当前复选框的状态
Qt::CheckState QCheckBox::checkState() const;


# 信号
// 当复选框的状态改变时，即当用户选中或取消选中复选框时，他的信号就会发出。
// 参数 state 表示的是复选框的三种状态中某一种, 可参考 Qt::CheckState
[signal] void QCheckBox::stateChanged(int state);
```
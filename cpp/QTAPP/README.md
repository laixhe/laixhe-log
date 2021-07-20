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
QVariant  
QPoint    坐标类
QLine     直线类
QSize     尺寸类
QRect     矩形类
QDate     日期类
QTime     时间类
QDateTime 日期和时间类
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

#### 自定义信号槽
> 所谓信号槽，实际就是观察者模式(发布-订阅模式)
>
> 如果想要使用自定义的信号槽，首先要编写新的类并且要继承 QObject 类或者其子类，还要在定义类的头文件中加入 QObject 中的 Q_OBJECT 这个宏
>

##### 自定义信号 signal
> 信号是类的成员函数
> 
> 要使用 signals 关键字进行声明，使用方法与 public 类似的关键字
>
> 返回值是 void 类型
> 
> 信号函数只需要声明，不需要定义(没有函数体实现)
> 
> 发送信号函数前加关键字： emit
> 

###### 自定义槽 slot
> 槽函数就是信号的处理动作，自定义槽函数和自定义的普通函数一样
>
> 要使用 slots 关键字进行声明，使用方法 public slots
> 
> 返回值是 void 类型
> 

#### QWidget 
> 是所有窗口类的父类(控件类是也属于窗口类), 并且 QWidget 类的父类的 QObject, 也就意味着所有的窗口类对象只要指定了父对象, 都可以实现内存资源的自动回收
>

```
// 构造函数
QWidget::QWidget(QWidget *parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

// 公共成员函数
// 给当前窗口设置父对象
void QWidget::setParent(QWidget *parent);
void QWidget::setParent(QWidget *parent, Qt::WindowFlags f);
// 获取当前窗口的父对象, 没有父对象返回 nullptr
QWidget *QWidget::parentWidget() const;

//------------- 窗口位置 -------------
// 得到相对于当前窗口父窗口的几何信息, 边框也被计算在内
QRect QWidget::frameGeometry() const;
// 得到相对于当前窗口父窗口的几何信息, 不包括边框
const QRect &geometry() const;
// 设置当前窗口的几何信息(位置和尺寸信息), 不包括边框
void setGeometry(int x, int y, int w, int h);
void setGeometry(const QRect &);
    
// 移动窗口, 重新设置窗口的位置
void move(int x, int y);
void move(const QPoint &);

//------------- 窗口尺寸 -------------
// 获取当前窗口的尺寸信息
QSize size() const
// 重新设置窗口的尺寸信息
void resize(int w, int h);
void resize(const QSize &);
// 获取当前窗口的最大尺寸信息
QSize maximumSize() const;
// 获取当前窗口的最小尺寸信息
QSize minimumSize() const;
// 设置当前窗口固定的尺寸信息
void QWidget::setFixedSize(const QSize &s);
void QWidget::setFixedSize(int w, int h);
// 设置当前窗口的最大尺寸信息
void setMaximumSize(const QSize &);
void setMaximumSize(int maxw, int maxh);
// 设置当前窗口的最小尺寸信息
void setMinimumSize(const QSize &);
void setMinimumSize(int minw, int minh);


// 获取当前窗口的高度    
int height() const;
// 获取当前窗口的最小高度
int minimumHeight() const;
// 获取当前窗口的最大高度
int maximumHeight() const;
// 给窗口设置固定的高度
void QWidget::setFixedHeight(int h);
// 给窗口设置最大高度
void setMaximumHeight(int maxh);
// 给窗口设置最小高度
void setMinimumHeight(int minh);

// 获取当前窗口的宽度
int width() const;
// 获取当前窗口的最小宽度
int minimumWidth() const;
// 获取当前窗口的最大宽度
int maximumWidth() const;
// 给窗口设置固定宽度
void QWidget::setFixedWidth(int w);
// 给窗口设置最大宽度
void setMaximumWidth(int maxw);
// 给窗口设置最小宽度
void setMinimumWidth(int minw);

//------------- 窗口图标 -------------
// 得到当前窗口的图标
QIcon windowIcon() const;
// 构造图标对象, 参数为图片的路径
QIcon::QIcon(const QString &fileName);
// 设置当前窗口的图标
void setWindowIcon(const QIcon &icon);

//------------- 窗口标题 -------------
// 得到当前窗口的标题
QString windowTitle() const;
// 设置当前窗口的标题
void setWindowTitle(const QString &);

// 判断窗口是否可用
bool isEnabled() const;
// 设置窗口是否可用, 不可用窗口无法接收和处理窗口事件
void setEnabled(bool);

//------------- 窗口显示 -------------
// 关闭当前窗口
[slot] bool QWidget::close();
// 隐藏当前窗口
[slot] void QWidget::hide();
// 显示当前创建以及其子窗口
[slot] void QWidget::show();
// 全屏显示当前窗口, 只对windows有效
[slot] void QWidget::showFullScreen();
// 窗口最大化显示, 只对windows有效
[slot] void QWidget::showMaximized();
// 窗口最小化显示, 只对windows有效
[slot] void QWidget::showMinimized();
// 将窗口回复为最大化/最小化之前的状态, 只对windows有效
[slot] void QWidget::showNormal();


//------------- 信号 -------------
// QWidget::setContextMenuPolicy(Qt::ContextMenuPolicy policy);
// 窗口的右键菜单策略 contextMenuPolicy() 参数设置为 Qt::CustomContextMenu, 按下鼠标右键发射该信号
[signal] void QWidget::customContextMenuRequested(const QPoint &pos);
// 窗口图标发生变化, 发射此信号
[signal] void QWidget::windowIconChanged(const QIcon &icon);
// 窗口标题发生变化, 发射此信号
[signal] void QWidget::windowTitleChanged(const QString &title);
```

#### QMainWindow
> 默认结构最复杂的标准窗口
> >
> > 提供了菜单栏, 工具栏, 状态栏, 停靠窗口
> >
> > 菜单栏: 只能有一个, 创建的最上方
> >
> > 工具栏: 可以有多个, 默认提供了一个, 窗口的上下左右都可以停靠
> >
> > 状态栏: 只能有一个, 窗口最下方
> >
> > 停靠窗口: 可以有多个, 默认没有提供, 窗口的上下左右都可以停靠
>

```
```


#### QDialog
> 对话框类是 QWidget 类的子类, 处理继承自父类的属性之外, 还有一些自己所特有的属性

```
// 构造函数
QDialog::QDialog(QWidget *parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

// 模态显示窗口
[virtual slot] int QDialog::exec();
// 隐藏模态窗口, 并且解除模态窗口的阻塞, 将 exec() 的返回值设置为 QDialog::Accepted
[virtual slot] void QDialog::accept();
// 隐藏模态窗口, 并且解除模态窗口的阻塞, 将 exec() 的返回值设置为 QDialog::Rejected
[virtual slot] void QDialog::reject();
// 关闭对话框并将其结果代码设置为r。finished()信号将发出r;
// 如果r是QDialog::Accepted 或 QDialog::Rejected，则还将分别发出accept()或Rejected()信号。
[virtual slot] void QDialog::done(int r);

[signal] void QDialog::accepted();
[signal] void QDialog::rejected();
[signal] void QDialog::finished(int result);

```

#### QMessageBox
> 是 QDialog 类的子类, 通过这个类可以显示一些简单的提示框, 用于展示警告、错误、问题等信息

```
// 显示一个模态对话框, 将参数 text 的信息展示到窗口中
[static] void QMessageBox::about(QWidget *parent, const QString &title, const QString &text);

/*
参数:
- parent: 对话框窗口的父窗口
- title: 对话框窗口的标题
- text: 对话框窗口中显示的提示信息
- buttons: 对话框窗口中显示的按钮(一个或多个)
- defaultButton
    1. defaultButton指定按下Enter键时使用的按钮。
    2. defaultButton必须引用在参数 buttons 中给定的按钮。
    3. 如果defaultButton是QMessageBox::NoButton, QMessageBox会自动选择一个合适的默认值。
*/
// 显示一个信息模态对话框
[static] QMessageBox::StandardButton QMessageBox::information(QWidget *parent, const QString &title, const QString &text, QMessageBox::StandardButtons buttons = Ok, QMessageBox::StandardButton defaultButton = NoButton);

// 显示一个错误模态对话框
[static] QMessageBox::StandardButton QMessageBox::critical(QWidget *parent, const QString &title, const QString &text, QMessageBox::StandardButtons buttons = Ok, QMessageBox::StandardButton defaultButton = NoButton);

// 显示一个问题模态对话框
[static] QMessageBox::StandardButton QMessageBox::question(QWidget *parent, const QString &title, const QString &text, QMessageBox::StandardButtons buttons = StandardButtons(Yes | No), QMessageBox::StandardButton defaultButton = NoButton);

// 显示一个警告模态对话框
[static] QMessageBox::StandardButton QMessageBox::warning(QWidget *parent, const QString &title, const QString &text, QMessageBox::StandardButtons buttons = Ok, QMessageBox::StandardButton defaultButton = NoButton);
```

#### QFileDialog
> 是 QDialog 类的子类, 通过这个类可以选择要打开/保存的文件或者目录

```
/*
通用参数:
	- parent: 当前对话框窗口的父对象也就是父窗口
	- caption: 当前对话框窗口的标题
	- dir: 当前对话框窗口打开的默认目录
	- options: 当前对话框窗口的一些可选项,枚举类型, 一般不需要进行设置, 使用默认值即可
	- filter: 过滤器, 在对话框中只显示满足条件的文件, 可以指定多个过滤器, 使用 ;; 分隔
		- 样式举例: 
			- Images (*.png *.jpg)
			- Images (*.png *.jpg);;Text files (*.txt)
	- selectedFilter: 如果指定了多个过滤器, 通过该参数指定默认使用哪一个, 不指定默认使用第一个过滤器
*/
// 打开一个目录, 得到这个目录的绝对路径
[static] QString QFileDialog::getExistingDirectory(QWidget *parent = nullptr, const QString &caption = QString(), const QString &dir = QString(), QFileDialog::Options options = ShowDirsOnly);

// 打开一个文件, 得到这个文件的绝对路径
[static] QString QFileDialog::getOpenFileName(QWidget *parent = nullptr, const QString &caption = QString(), const QString &dir = QString(), const QString &filter = QString(), QString *selectedFilter = nullptr, QFileDialog::Options options = Options());

// 打开多个文件, 得到这多个文件的绝对路径
[static] QStringList QFileDialog::getOpenFileNames(QWidget *parent = nullptr, const QString &caption = QString(), const QString &dir = QString(), const QString &filter = QString(), QString *selectedFilter = nullptr, QFileDialog::Options options = Options());

// 打开一个目录, 使用这个目录来保存指定的文件
[static] QString QFileDialog::getSaveFileName(QWidget *parent = nullptr, const QString &caption = QString(), const QString &dir = QString(), const QString &filter = QString(), QString *selectedFilter = nullptr, QFileDialog::Options options = Options());
```

#### QFontDialog
> QFont 字体类

```
/*
参数:
	- family: 本地字库中的字体名, 通过 office 等文件软件可以查看
	- pointSize: 字体的字号
	- weight: 字体的粗细, 有效范围为 0 ~ 99
	- italic: 字体是否倾斜显示, 默认不倾斜
*/
QFont::QFont(const QString &family, int pointSize = -1, int weight = -1, bool italic = false);

// 设置字体
void QFont::setFamily(const QString &family);
// 根据字号设置字体大小
void QFont::setPointSize(int pointSize);
// 根据像素设置字体大小
void QFont::setPixelSize(int pixelSize);
// 设置字体的粗细程度, 有效范围: 0 ~ 99
void QFont::setWeight(int weight);
// 设置字体是否加粗显示
void QFont::setBold(bool enable);
// 设置字体是否要倾斜显示
void QFont::setItalic(bool enable);

// 获取字体相关属性(一般规律: 去掉设置函数的 set 就是获取相关属性对应的函数名)
QString QFont::family() const;
bool QFont::italic() const;
int QFont::pixelSize() const;
int QFont::pointSize() const;
bool QFont::bold() const;
int QFont::weight() const;
```

> QFontDialog 类的静态API

```
[static] QFont QFontDialog::getFont(bool *ok, const QFont &initial, QWidget *parent = nullptr, const QString &title = QString(), QFontDialog::FontDialogOptions options = FontDialogOptions());

[static] QFont QFontDialog::getFont(bool *ok, QWidget *parent = nullptr);
```

> 窗口字体的设置

```
// QWidget 类
// 得到当前窗口使用的字体
const QWidget::QFont& font() const;
// 给当前窗口设置字体, 只对当前窗口类生效
void QWidget::setFont(const QFont &);

// QApplication 类
// 得到当前应用程序对象使用的字体
[static] QFont QApplication::font();
// 给当前应用程序对象设置字体, 作用于当前应用程序的所有窗口
[static] void QApplication::setFont(const QFont &font, const char *className = nullptr);

```

#### QColorDialog
> 颜色类 QColor

```
// 构造函数
QColor::QColor(Qt::GlobalColor color);
QColor::QColor(int r, int g, int b, int a = ...);
QColor::QColor();

// 参数 red, green, blue, alpha 取值范围是 0-255
void QColor::setRed(int red);
void QColor::setGreen(int green);
void QColor::setBlue(int blue);
void QColor::setAlpha(int alpha);
void QColor::setRgb(int r, int g, int b, int a = 255);

int QColor::red() const;
int QColor::green() const;
int QColor::blue() const;
int QColor::alpha() const;
void QColor::getRgb(int *r, int *g, int *b, int *a = nullptr) const;
```

> QColorDialog 类的静态API

```
// 弹出颜色选择对话框, 并返回选中的颜色信息
[static] QColor QColorDialog::getColor(const QColor &initial = Qt::white, QWidget *parent = nullptr, const QString &title = QString(), QColorDialog::ColorDialogOptions options = ColorDialogOptions());

```

#### QProgressDialog
```
// 构造函数
/*
参数:
	- labelText: 对话框中显示的提示信息
	- cancelButtonText: 取消按钮上显示的文本信息
	- minimum: 进度条最小值
	- maximum: 进度条最大值
	- parent: 当前窗口的父对象
	- f: 当前进度窗口的flag属性, 使用默认属性即可, 无需设置
*/
QProgressDialog::QProgressDialog(QWidget *parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());
QProgressDialog::QProgressDialog(const QString &labelText, const QString &cancelButtonText, int minimum, int maximum, QWidget *parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());


// 设置取消按钮显示的文本信息
[slot] void QProgressDialog::setCancelButtonText(const QString &cancelButtonText);

// 公共成员函数和槽函数
QString QProgressDialog::labelText() const;
void QProgressDialog::setLabelText(const QString &text);

// 得到进度条最小值
int QProgressDialog::minimum() const;
// 设置进度条最小值
void QProgressDialog::setMinimum(int minimum);

// 得到进度条最大值
int QProgressDialog::maximum() const;
// 设置进度条最大值
void QProgressDialog::setMaximum(int maximum);

// 设置进度条范围(最大和最小值)
[slot] void QProgressDialog::setRange(int minimum, int maximum);

// 得到进度条当前的值
int QProgressDialog::value() const;
// 设置进度条当前的值
void QProgressDialog::setValue(int progress);


bool QProgressDialog::autoReset() const;
// 当value() = maximum()时，进程对话框是否调用reset()，此属性默认为true。
void QProgressDialog::setAutoReset(bool reset);


bool QProgressDialog::autoClose() const;
// 当value() = maximum()时，进程对话框是否调用reset()并且隐藏，此属性默认为true。
void QProgressDialog::setAutoClose(bool close);

// 判断用户是否按下了取消键, 按下了返回true, 否则返回false
bool wasCanceled() const;


// 重置进度条
// 重置进度对话框。wascancelled()变为true，直到进程对话框被重置。进度对话框被隐藏。
[slot] void QProgressDialog::cancel();
// 重置进度对话框。如果autoClose()为真，进程对话框将隐藏。
[slot] void QProgressDialog::reset();   

// 信号
// 当单击cancel按钮时，将发出此信号。默认情况下，它连接到cancel()槽。
[signal] void QProgressDialog::canceled();

// 设置窗口的显示状态(模态, 非模态)
/*
参数:
	Qt::NonModal  -> 非模态
	Qt::WindowModal	-> 模态, 阻塞父窗口
	Qt::ApplicationModal -> 模态, 阻塞应用程序中的所有窗口
*/
void QWidget::setWindowModality(Qt::WindowModality windowModality);

```

> QMainWindow 是标准基础窗口中结构最复杂的窗口
> >
> > 提供了菜单栏 , 工具栏 , 状态栏 , 停靠窗口
> >
> > 菜单栏：只能有一个，位于窗口的最上方
> >
> > 工具栏：可以有多个，默认提供了一个，窗口的上下左右都可以停靠
> >
> > 状态栏：只能有一个，位于窗口最下方
> >
> > 停靠窗口：可以有多个，默认没有提供，窗口的上下左右都可以停靠
> >

#### 菜单栏

##### 通过代码的方式
```
// 给菜单栏添加菜单
QAction *QMenuBar::addMenu(QMenu *menu);
QMenu *QMenuBar::addMenu(const QString &title);
QMenu *QMenuBar::addMenu(const QIcon &icon, const QString &title);

// 给菜单对象添加菜单项(QAction)
QAction *QMenu::addAction(const QString &text);
QAction *QMenu::addAction(const QIcon &icon, const QString &text);

// 添加分割线
QAction *QMenu::addSeparator();
```

##### 菜单项 QAction 事件的处理
> 单击菜单项，该对象会发出一个信号

```
# 点击 QAction 对象发出该信号
[signal] void QAction::triggered(bool checked = false);

# 示例代码
// save_action 是某个菜单项对象名, 点击这个菜单项会弹出一个对话框
connect(ui->save_action, &QAction::triggered, this, [=](){
      QMessageBox::information(this, "Triggered", "我是菜单项, 你不要调戏我...");
});
```

#### 工具栏

##### 通过代码的方式
```
// 在QMainWindow窗口中添加工具栏
void QMainWindow::addToolBar(Qt::ToolBarArea area, QToolBar *toolbar);
void QMainWindow::addToolBar(QToolBar *toolbar);
QToolBar *QMainWindow::addToolBar(const QString &title);

// 将Qt控件放到工具栏中
// 工具栏类: QToolBar
// 添加的对象只要是QWidget或者启子类都可以被添加
QAction *QToolBar::addWidget(QWidget *widget);

// 添加QAction对象
QAction *QToolBar::addAction(const QString &text);
QAction *QToolBar::addAction(const QIcon &icon, const QString &text);

// 添加分隔线
QAction *QToolBar::addSeparator()
```

```
MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // 添加第二个工具栏
    QToolBar* toolbar = new QToolBar("toolbar");
    this->addToolBar(Qt::LeftToolBarArea, toolbar);

    // 给工具栏添加按钮和单行输入框
    ui->toolBar->addWidget(new QPushButton("搜索"));
    QLineEdit* edit = new QLineEdit;
    edit->setMaximumWidth(200);
    edit->setFixedWidth(100);
    ui->toolBar->addWidget(edit);
    // 添加QAction类型的菜单项
    ui->toolBar->addAction(QIcon(":/er-dog"), "二狗子");
}
```

#### 状态栏
> 一般情况下，需要在状态栏中添加某些控件，显示某些属性，使用最多的就是添加标签 QLabel

##### 通过代码的方式
```
// 类型: QStatusBar
void QStatusBar::addWidget(QWidget *widget, int stretch = 0);

[slot] void QStatusBar::clearMessage();
[slot] void QStatusBar::showMessage(const QString &message, int timeout = 0);
```

```
MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // 状态栏添加子控件
    // 按钮
    QPushButton* button = new QPushButton("按钮");
    ui->statusBar->addWidget(button);
    // 标签
    QLabel* label = new QLabel("hello,world");
    ui->statusBar->addWidget(label);
}
```

#### 鼠标右键菜单

##### 基于鼠标事件实现

> 1.在当前窗口类中重写鼠标操作相关的的事件处理器函数

```
// 以下两个事件二选一即可, 只是事件函数被调用的时机不同罢了
// 这个时机对右键菜单的显示没有任何影响
[virtual protected] void QWidget::mousePressEvent(QMouseEvent *event);
[virtual protected] void QWidget::mouseReleaseEvent(QMouseEvent *event);
```

> 2.在数据表事件处理器函数内部判断是否按下了鼠标右键

> 3.如果按下了鼠标右键创建菜单对象 (也可以提前先创建处理), 并将其显示出来
```
// 关于QMenu类型的菜单显示需要调用的 API
// 参数 p 就是右键菜单需要显示的位置, 这个坐标需要使用屏幕坐标
// 该位置坐标一般通过调用 QCursor::pos() 直接就可以得到了
QAction *QMenu::exec(const QPoint &p, QAction *action = nullptr);
```

> 4.代码实现
```
# 在头文件中添加要重写的鼠标事件处理器函数声明，这里使用的是 mousePressEvent()

// mainwindow.h
#include <QMainWindow>

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = 0);
    ~MainWindow();

protected:
    // 鼠标按下, 该函数被Qt框架调用, 需要重写该函数
    void mousePressEvent(QMouseEvent *event);

private:
    Ui::MainWindow *ui;
};

# 在源文件中重写从父类继承的虚函数 mousePressEvent()

// mainwindow.cpp
void MainWindow::mousePressEvent(QMouseEvent *event)
{
    // 判断用户按下的是哪一个鼠标键
    if(event->button() == Qt::RightButton)
    {
        // 弹出一个菜单, 菜单项是 QAction 类型
        QMenu menu;
        QAction* act = menu.addAction("C++");
        connect(act, &QAction::triggered, this, [=]()
        {
            QMessageBox::information(this, "title", "您选择的是C++...");
        });
        menu.addAction("Java");
        menu.addAction("Python");
        menu.exec(QCursor::pos()); // 右键菜单被模态显示出来了
    }
}

```

#### 基于窗口的菜单策略实现
> 使用 Qt 中 QWidget 类中的右键菜单函数 QWidget::setContextMenuPolicy(Qt::ContextMenuPolicy policy) 来实现

```
// 函数原型:
void QWidget::setContextMenuPolicy(Qt::ContextMenuPolicy policy);
参数: 	
  - Qt::NoContextMenu	     --> 不能实现右键菜单
  - Qt::PreventContextMenu   --> 不能实现右键菜单
  - Qt::DefaultContextMenu   --> 基于事件处理器函数 QWidget::contextMenuEvent() 实现
  - Qt::ActionsContextMenu   --> 添加到当前窗口中所有 QAction 都会作为右键菜单项显示出来
  - Qt::CustomContextMenu    --> 基于 QWidget::customContextMenuRequested() 信号实现
```

##### Qt::DefaultContextMenu 
> 使用这个策略实现右键菜单，需要借助窗口类从父类继承的虚函数 QWidget::contextMenuEvent() 并重写它来实现

```
# 头文件

// mainwindow.h
#include <QMainWindow>

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = 0);
    ~MainWindow();

protected:
    // 如果窗口设置了 Qt::DefaultContextMenu 策略, 
    // 点击鼠标右键该函数被Qt框架调用
    void contextMenuEvent(QContextMenuEvent *event);

private:
    Ui::MainWindow *ui;
};

# 源文件

// mainwindow.cpp
MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // 给窗口设置策略: Qt::DefaultContextMenu
    // 在窗口中按下鼠标右键, 这个事件处理器函数被qt框架调用 QWidget::contextMenuEvent()
    setContextMenuPolicy(Qt::DefaultContextMenu);
}

// 重写事件处理器函数 contextMenuEvent()
void MainWindow::contextMenuEvent(QContextMenuEvent *event)
{
    // 弹出一个菜单, 菜单项是 QAction 类型
    QMenu menu;
    QAction* act = menu.addAction("C++");
    connect(act, &QAction::triggered, this, [=]()
    {
        QMessageBox::information(this, "title", "您选择的是C++...");
    });
    menu.addAction("Java");
    menu.addAction("Python");
    menu.exec(QCursor::pos());	// 右键菜单被模态显示出来了
}

```

##### Qt::CustomContextMenu
> 使用这个策略实现右键菜单，当点击鼠标右键，窗口会产生一个 QWidget::customContextMenuRequested() 信号，注意仅仅只是发射信号，意味着要自己写显示右键菜单的槽函数（slot），这个信号是 QWidget 唯一与右键菜单有关的信号

```
# 注意: 信号中的参数pos为当前窗口的坐标，并非屏幕坐标，右键菜单显示需要使用屏幕坐标
[signal] void QWidget::customContextMenuRequested(const QPoint &pos)

// mainwindow.cpp
MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // 策略 Qt::CustomContextMenu
    // 当在窗口中点击鼠标右键, 窗口会发出一个信号: QWidget::customContextMenuRequested()
    // 对应发射出的这个信号, 需要添加一个槽函数, 用来显示右键菜单
    this->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(this, &MainWindow::customContextMenuRequested, this, [=](const QPoint &pos)
    {
        // 参数 pos 是鼠标按下的位置, 但是不能直接使用, 这个坐标不是屏幕坐标, 是当前窗口的坐标
        // 如果要使用这个坐标需要将其转换为屏幕坐标
        QMenu menu;
        QAction* act = menu.addAction("C++");
        connect(act, &QAction::triggered, this, [=]()
        {
            QMessageBox::information(this, "title", "您选择的是C++...");
        });
        menu.addAction("Java");
        menu.addAction("Python");
        // menu.exec(QCursor::pos());
        // 将窗口坐标转换为屏幕坐标
        QPoint newpt = this->mapToGlobal(pos);
        menu.exec(newpt);
    });
}
```

> 在上边的程序中，我们通过窗口发射的信号得到了一个坐标类型的参数，大家一定要注意这个坐标是当前窗口的窗口坐标, 不是屏幕坐标, 显示右键菜单需要使用屏幕坐标。
>
> 对应这个坐标的处理可以有两种方式：
> >
> > 弃用，选择使用 QCursor::pos() 得到光标在屏幕的坐标位置
> > 
> > 坐标转换，将窗口坐标转换为屏幕坐标，这里用到了一个函数 QPoint QWidget::mapToGlobal(const QPoint &pos) const
> > 

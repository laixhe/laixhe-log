#include "testwidget.h"
#include "ui_testwidget.h"

// QWidget
// 所有窗口类的基类
// 可以内嵌到其它窗口的内部-无边框(需要给该窗口指定父窗口)
// 可以作为独立的窗口显示-有边框(不能给该窗口指定父窗口)
// 是 Qt 中所有控件的基类


// 隐藏标题栏
//this->setWindowFlags(Qt::CustomizeWindowHint);
// 设置窗口的最大尺寸
//setMaximumSize(600, 600);
// 设置窗口的最小尺寸
//setMinimumSize(300, 300);
// 设置窗口的固定尺寸
//setFixedSize(500, 500);
// 设置标题
//setWindowTitle("xx");
// 设置窗口的图标(左上角的)
//setWindowIcon(QIcon("./xx.png"));
// 移动(修改)窗口位置
//move(10, 10);
// 重新设置窗口的宽高
//resize
//
// 获取当前窗口的位置包括边框
//QRect qrect = frameGeometry();
// 获取当前窗口的宽高
// size()
//
// 窗口的右键菜单(按下鼠标右键)-发射该信号
// QWidget::setContextMenuPolicy(Qt::ContextMenuPolicy policy)
// void QWidget::customContextMenuRequested(const QPoint &pos) // ( connect(this, &MainWindow::customContextMenuRequested, this, []{}) )
// 窗口图标发生变化-发射该信号 ( connect(this, &MainWindow::windowIconChanged, this, []{}) )
// void QWidget::windowIconChanged(const QIcon &icon)
// 窗口标题发生变化-发射该信号 ( connect(this, &MainWindow::windowTitleChanged, this, []{}) )
// void QWidget::windowTitleChanged(const QString &title)

TestWidget::TestWidget(QWidget *parent) : QWidget(parent), ui(new Ui::TestWidget) {
    ui->setupUi(this);
}

TestWidget::~TestWidget() {
    delete ui;
}

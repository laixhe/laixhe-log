#include "testwidget.h"
#include "ui_testwidget.h"

// QWidget
// 可以内嵌到其它窗口的内部-无边框(需要给该窗口指定父窗口)
// 可以作为独立的窗口显示-有边框(不能给该窗口指定父窗口)
// 是所有窗口类的父类(控件类是也属于窗口类), 并且 QWidget 类的父类的 QObject, 也就意味着所有的窗口类对象只要指定了父对象, 都可以实现内存资源的自动回收

TestWidget::TestWidget(QWidget *parent) : QWidget(parent), ui(new Ui::TestWidget) {
    ui->setupUi(this);
}

TestWidget::~TestWidget() {
    delete ui;
}

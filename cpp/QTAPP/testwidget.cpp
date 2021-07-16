#include "testwidget.h"
#include "ui_testwidget.h"

// QWidget
// 所有窗口类的基类
// 可以内嵌到其它窗口的内部-无边框(需要给该窗口指定父窗口)
// 可以作为独立的窗口显示-有边框(不能给该窗口指定父窗口)
// 是 Qt 中所有控件的基类

TestWidget::TestWidget(QWidget *parent) : QWidget(parent), ui(new Ui::TestWidget) {
    ui->setupUi(this);
}

TestWidget::~TestWidget() {
    delete ui;
}

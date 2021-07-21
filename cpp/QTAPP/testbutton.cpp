#include "testbutton.h"
#include "ui_testbutton.h"

#include <QMenu>

// QAbstractButton 是所有按钮的基类

// QPushButton
// 这个类是一个常用按钮类, 操作这个按钮使用的大部分函数都是从父类继承过来的, 它的父类是QAbstractButton
//
// QToolButton 增强版按钮
// 使用方法和功能跟 QPushButton 基本一致, 只不过在和菜单进行管理这个方面,QToolButton 类可以设置弹出的菜单的属性,
// 以及在显示图标的时候可以设置更多的样式, 可以理解为是一个增强版的 QPushButton
//
// QCheckBox (复选框(多选)按钮)
// 是 Qt 中的复选框按钮，可以单独使用，也可以以组的方式使用 (同一组可以同时选中多个)
//
// QRadioButton (单选按钮)
// 是 Qt 提供的单选按钮，一般都是`以组`的方式来使用 (多个按钮中同时只能选中其中一个)

TestButton::TestButton(QWidget *parent) : QDialog(parent), ui(new Ui::TestButton) {
    ui->setupUi(this);

    // 普通按钮, 没有checked属性
    ui->normalBtn->setText("按钮普通");
    // 设置图片
    ui->normalBtn->setIcon(QIcon(":/res/images/tab_info.png"));
    // 连接 clicked 信号(点击事件)
    connect(ui->normalBtn, &QPushButton::clicked, this, [&]() {
        qDebug() << "我是一个普通按钮, 没有checked属性...";
    });

    // 有checked属性的按钮(开关按钮)
    ui->checkedBtn->setCheckable(true);
    connect(ui->checkedBtn, &QPushButton::toggled, this, [&](bool bl){
        qDebug() << "我是一个checked按钮, 当前状态为:" << bl;
    });

    // 普通菜单按钮
    QMenu* menu = new QMenu;
    QAction* act = menu->addAction("菜单1");
    menu->addAction("菜单2");
    menu->addAction("菜单3");
    menu->addAction("菜单4");
    ui->menuBtn->setMenu(menu);
    connect(act, &QAction::triggered, this, [&]{
        qDebug() << "点击了-菜单1";
    });

    // 增强版菜单按钮
}

TestButton::~TestButton() {
    delete ui;
}

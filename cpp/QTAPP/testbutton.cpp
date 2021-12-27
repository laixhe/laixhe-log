#include "testbutton.h"
#include "ui_testbutton.h"

#include <QMenu>
#include <QDebug>

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

    // 普通按钮 QPushButton

    // 设置图片
    ui->normalBtn->setIcon(QIcon(":/res/images/tab_info.png"));
    // 连接 clicked 信号(点击事件)
    connect(ui->normalBtn, &QPushButton::clicked, this, [&]() {
        qDebug() << "普通按钮, 没有checked属性...";
    });

    // 有checked属性的按钮(开关按钮)
    ui->checkedBtn->setCheckable(true);
    connect(ui->checkedBtn, &QPushButton::toggled, this, [&](bool bl){
        qDebug() << "普通按钮checked按钮, 当前状态为:" << bl;
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

    // 增强版按钮 QToolButton

    // 设置图片
    ui->toolNormalBtn->setIcon(QIcon(":/res/images/tab_mine.png"));
    // 设置按钮的文本与图标的位置，默认只显示图标
    ui->toolNormalBtn->setToolButtonStyle(Qt::ToolButtonTextBesideIcon);
    // 连接 clicked 信号(点击事件)
    connect(ui->toolNormalBtn, &QToolButton::clicked, this, [&]() {
        qDebug() << "增强版普通按钮, 没有checked属性...";
    });

    // 通过 QAction 方式设置一个按钮
    QAction *actBtn = new QAction(QIcon(":/res/images/tab_money.png"), "增强版按钮");
    ui->toolNormalActBtn->setDefaultAction(actBtn);
    connect(ui->toolNormalActBtn, &QToolButton::triggered, this, [&](QAction *act){
        act->setText("增强版按钮...");
        qDebug() << "增强版普通按钮QAction, 没有checked属性...";
    });
    ui->toolNormalActBtn->setToolButtonStyle(Qt::ToolButtonTextUnderIcon);

    // 箭头按钮
    ui->toolArrowBut->setArrowType(Qt::UpArrow);
    ui->toolArrowBut->setToolButtonStyle(Qt::ToolButtonTextUnderIcon);

    // 有checked属性的按钮(开关按钮)
    ui->toolCheckedBtn->setCheckable(true);
    connect(ui->toolCheckedBtn, &QToolButton::toggled, this, [&](bool bl){
        qDebug() << "增强版普通按钮checked按钮, 当前状态为:" << bl;
    });

    // 增强版菜单按钮
    QMenu* toolMenu = new QMenu;
    QAction* toolAct1 = toolMenu->addAction("增强版菜单1");
    toolMenu->addAction("增强版菜单2");
    toolMenu->addAction("增强版菜单3");
    toolMenu->addAction("增强版菜单4");
    connect(toolAct1, &QAction::triggered, this, [&]{
        qDebug() << "点击了-增强版菜单1";
    });

    /*
    弹出菜单的弹出模式是一个枚举类型: QToolButton::ToolButtonPopupMode, 取值如下:
        - QToolButton::DelayedPopup: 
            - 延时弹出, 按压工具按钮一段时间后才能弹出, 比如:浏览器的返回按钮
            - 长按按钮菜单弹出, 但是按钮的 clicked 信号不会被发射(默认的)
        - QToolButton::MenuButtonPopup: 
            - 在这种模式下，工具按钮会显示一个特殊的箭头，表示有菜单。
        - 当按下按钮的箭头部分时，将显示菜单。按下按钮部分发射 clicked 信号
        - QToolButton::InstantPopup: 
            - 当按下工具按钮时，菜单立即显示出来。
            - 在这种模式下，按钮本身的动作不会被触发(不会发射clicked信号)
    */

    // 增强版延时菜单按钮
    ui->toolMenuBtn->setMenu(toolMenu);
    // 增强版即时菜单按钮
    ui->toolPopMenuBtn->setMenu(toolMenu);
    ui->toolPopMenuBtn->setPopupMode(QToolButton::InstantPopup);

    // 多选-三态设置
    auto statusChanged = [&](int state){
        bool ishtml = ui->htmlCheckBox->isChecked();
        bool iscss = ui->cssCheckBox->isChecked();
        bool isjs = ui->jsCheckBox->isChecked();

        if(ishtml && iscss && isjs){
            // 如果三个都选中，那就设置 html 为选中状态
            ui->webCheckBox->setCheckState(Qt::Checked);
        }else if(!ishtml && !iscss && !isjs){
            // 如果三个都不选中，那就设置 html 为不选中状态
            ui->webCheckBox->setCheckState(Qt::Unchecked);
        } else {
            // 如果选择其中一个，那就设置 html 为半选中状态
            ui->webCheckBox->setCheckState(Qt::PartiallyChecked);
        }

        qDebug() << "htmlCheckBox:" << ui->htmlCheckBox->isChecked();
        qDebug() << "cssCheckBox:" << ui->cssCheckBox->isChecked();
        qDebug() << "jsCheckBox:" << ui->jsCheckBox->isChecked();
    };
    // 设置根节点的三态属性
    ui->webCheckBox->setTristate(true); // 默认就是 true
    connect(ui->htmlCheckBox, &QCheckBox::stateChanged, this, statusChanged);
    connect(ui->cssCheckBox, &QCheckBox::stateChanged, this, statusChanged);
    connect(ui->jsCheckBox, &QCheckBox::stateChanged, this, statusChanged);
}

TestButton::~TestButton() {
    delete ui;
}

void TestButton::on_radioCppBut_clicked(){
    qDebug() << "点击了 C++";
}


void TestButton::on_radioGoBut_clicked(){
    qDebug() << "点击了 Go";
}


void TestButton::on_radioPhpBut_clicked(){
    qDebug() << "点击了 Php";
}


void TestButton::on_radioJavaBut_clicked(){
    qDebug() << "点击了 Java";
}


void TestButton::on_radioPyBut_clicked(){
    qDebug() << "点击了 Python";
}


void TestButton::on_radioRustBut_clicked(){
    qDebug() << "点击了 Rust";
}


void TestButton::on_cppCheckBox_stateChanged(int arg1){
    qDebug() << "点击了 CPP";
}


void TestButton::on_goCheckBox_stateChanged(int arg1){
    qDebug() << "点击了 GOLANG";
}


void TestButton::on_phpCheckBox_stateChanged(int arg1){
    qDebug() << "点击了 PHP";
}


void TestButton::on_selectBut_clicked() {
    bool iscpp = ui->radioCppBut->isChecked();
    bool isgo = ui->radioGoBut->isChecked();
    bool isphp = ui->radioPhpBut->isChecked();

    bool isjava = ui->radioJavaBut->isChecked();
    bool ispython = ui->radioPyBut->isChecked();
    bool isrust = ui->radioRustBut->isChecked();

    bool isCPP = ui->cppCheckBox->isChecked();
    bool isGO = ui->goCheckBox->isChecked();
    bool isPHP = ui->phpCheckBox->isChecked();

    QString str("单选1: ");
    if(iscpp){
        str.append("cpp ");
    }
    if(isgo){
        str.append("go ");
    }
    if(isphp){
        str.append("php ");
    }

    str.append("\n单选2: ");
    if(isjava){
        str.append("java ");
    }
    if(ispython){
        str.append("python ");
    }
    if(isrust){
        str.append("rust ");
    }

    str.append("\n多选: ");
    if(isCPP){
        str.append("CPP ");
    }
    if(isGO){
        str.append("GO ");
    }
    if(isPHP){
        str.append("PHP ");
    }

    ui->selectLabel->setText(str);
}


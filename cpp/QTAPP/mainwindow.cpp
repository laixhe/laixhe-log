#include <iostream>

#include "mainwindow.h"
#include "ui_mainwindow.h"

#include <QMessageBox>
#include <QFileDialog>
#include <QPushButton>
#include <QDebug>

#include "testwidget.h"
#include "testdialog.h"

// QMainWindow
// 主窗口类
// 可以包含菜单栏、工具栏、状态栏
// 不能内嵌

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);

    m_signalsFrom = new SignalsFrom;
    m_signalsTo = new SignalsTo;

    // 创建窗口对象，没有给 w 对象指定父对象
    // 这个窗口是一个独立窗口 (只有独立的窗口才边框)
    //TestWidget *w = new TestWidget;
    // 显示窗口
    //w->show();

    // 创建自定义窗口对象，给对象指定父对象
    // 这个窗口是内嵌到父窗口中的 (没有边框的)
    //TestWidget *tWidget = new TestWidget(this);
    //tWidget->show();

    // 创建自定义对话框窗口
    //TestDialog *tDialog = new TestDialog();
    //tDialog->show(); // 显示对话窗口(非模态窗口)
    //tDialog->exec(); // 显示对话窗口(模态窗口)并阻塞程序,当前窗口有焦点 其它 窗口失去焦点
    
    // 创建按钮，让这个按钮作为当前创建的子控件
    //QPushButton *btnA = new QPushButton(this);
    // 移动按钮的位置
    //btnA->move(10, 10);
    // 给按钮设置固定大小
    //btnA->setFixedSize(150, 150);

    qDebug() << QString("A(%1)A").arg("字符串");

    // 连接自定义信号槽 - 等待触发 (相当于先关联好)
    connect(m_signalsFrom, &SignalsFrom::testFrom, m_signalsTo, &SignalsTo::testTo);
    // 连接自定义槽 - 触发自定义信号按钮(点击事件)
    //connect(ui->signalsFromTo, &QPushButton::clicked, this, &MainWindow::on_signalsFromTo_clicked);

    // 连接标准信号槽(点击(关闭窗口)按钮(点击事件) -> 关闭当前窗口) - 等待触发 (相当于先关联好)
    connect(ui->closeBtn, &QPushButton::clicked, this, &MainWindow::close);

    // 窗口的右键菜单(按下鼠标右键)(点击事件)
    setContextMenuPolicy(Qt::CustomContextMenu);
    connect(this, &MainWindow::customContextMenuRequested, this, [=](const QPoint &pos){
        QMenu menu;
        menu.addAction("右键菜单1");
        menu.addAction("右键菜单2");
        menu.addAction("右键菜单3");
        menu.exec(QCursor::pos());
    });

}

// 用于-触发自定义信号按钮(点击事件)
void MainWindow::on_signalsFromTo_clicked() {
    // 触发自定义信号
    emit m_signalsFrom->testFrom();
}
// 开打Test对话框按钮(点击事件)
void MainWindow::on_testDialogBut_clicked(){

    // 创建自定义对话框窗口
    TestDialog tDialog;
    int ret = tDialog.exec(); // 显示对话窗口(模态窗口)并阻塞程序,当前窗口有焦点 其它 窗口失去焦点
    if(ret == QDialog::Accepted) {
        qDebug() << "accept buttion clicked...";
        std::cout << "...accept buttion clicked..." << ret << std::endl;
    } else if(ret == QDialog::Rejected) {
        qDebug() << "rejected buttion clicked...";
        std::cout << "...rejected buttion clicked..." << ret << std::endl;
    } else {
        qDebug() << "done buttion clicked...";
        std::cout << "...done buttion clicked..." << ret << std::endl;
    }

}

// 开打对话框按钮(点击事件)
void MainWindow::on_messageDialogBut_clicked(){
    //QMessageBox::about(this, "提示", "提示信息...");
    //QMessageBox::information(this, "通知", "通知消息...");
    //QMessageBox::critical(this, "错误", "错误信息...");
    //QMessageBox::warning(this, "警告", "是否关闭？");
    //QMessageBox::question(this, "询问", "询问提示信息...");

     int ret = QMessageBox::question(this, "保存", "你是否要保存信息...", QMessageBox::Save|QMessageBox::Cancel, QMessageBox::Cancel);
     if(ret == QMessageBox::Save){
         QMessageBox::information(this, "通知", "保存成功...");
     }
     if(ret == QMessageBox::Cancel){
         QMessageBox::warning(this, "警告", "你放弃了保存...");
     }

}
// 开打文件对话框按钮(点击事件)
void MainWindow::on_fileDialogBut_clicked() {
}

MainWindow::~MainWindow() {
    delete ui;

    delete m_signalsFrom;
    delete m_signalsTo;
}

/*
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlQuery>

QSqlDatabase db = QSqlDatabase::addDatabase("QMYSQL");
db.setHostName("192.168.10.240");
db.setDatabaseName("byx");
db.setUserName("root");
db.setPassword("123456");
bool ok = db.open();
if(!ok){
    std::cout << "db open error..." << std::endl;
    return;
}

QSqlQuery query("select `id`,`log` from `migrate_db`", db);
while (query.next()) {
    QString id = query.value(0).toString();
    QString log = query.value(1).toString();

    std::cout << id.toStdString() << ":" << log.toStdString() << std::endl;
}
db.close();
*/


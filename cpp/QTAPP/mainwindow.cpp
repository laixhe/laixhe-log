#include "mainwindow.h"
#include "ui_mainwindow.h"

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

    // 隐藏标题栏
    //this->setWindowFlags(Qt::CustomizeWindowHint);

    // 创建窗口对象，没有给 w 对象指定父对象
    // 这个窗口是一个独立窗口 (只有独立的窗口才边框)
    //TestWidget *w = new TestWidget;
    // 显示窗口
    //w->show();

    // 创建窗口对象，给对象指定父对象
    // 这个窗口是内嵌到父窗口中的 (没有边框的)
    //TestWidget *tWidget = new TestWidget(this);
    //tWidget->show();

    // 创建对话框窗口
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

    //
    connect(ui->closeBtn, &QPushButton::clicked, this, &MainWindow::close);
}

MainWindow::~MainWindow() {
    delete ui;
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

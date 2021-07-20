#include <iostream>

#include "mainwindow.h"
#include "ui_mainwindow.h"

#include <QMessageBox>
#include <QFileDialog>
#include <QFontDialog>
#include <QColorDialog>
#include <QPainter>

#include "testwidget.h"
#include "testdialog.h"

// QMainWindow
// 主窗口类
// 可以包含菜单栏、工具栏、状态栏、停靠窗口
// 不能内嵌
//
// 菜单栏: 只能有一个, 创建的最上方
// 工具栏: 可以有多个, 默认提供了一个, 窗口的上下左右都可以停靠
// 状态栏: 只能有一个, 窗口最下方
// 停靠窗口: 可以有多个, 默认没有提供, 窗口的上下左右都可以停靠
//
MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);
    // 给窗口设置图标
    setWindowIcon(QIcon(":/res/images/00.png"));

    m_signalsFrom = new SignalsFrom;
    m_signalsTo = new SignalsTo;

    // 连接菜单栏信号槽(点击事件)
    connect(ui->fileOpenAction, &QAction::triggered, this, [&](){
        QMessageBox::information(this, "菜单文件", "打开菜单文件...");
    });
    connect(ui->fileSaveAction, &QAction::triggered, this, [&](){
        QMessageBox::information(this, "菜单文件", "保存菜单文件...");
    });

    // 给工具栏添加按钮和单行输入框
    ui->toolBar->addWidget(new QPushButton("搜索"));
    ui->toolBar->addWidget(new QLineEdit());


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
    // QFileDialog 通用参数
    // parent: 当前对话框窗口的父对象也就是父窗口
    // caption: 当前对话框窗口的标题
    // dir: 当前对话框窗口打开的默认目录
    // options: 当前对话框窗口的一些可选项,枚举类型, 一般不需要进行设置, 使用默认值即可
    // filter: 过滤器, 在对话框中只显示满足条件的文件, 可以指定多个过滤器, 使用 ;; 分隔
    //         样式举例:
    //                Images (*.png *.jpg)
    //                Images (*.png *.jpg);;Text files (*.txt)
    // selectedFilter: 如果指定了多个过滤器, 通过该参数指定默认使用哪一个, 不指定默认使用第一个过滤器
    //
    // 打开目录，返回绝对路径
    //QString dirName = QFileDialog::getExistingDirectory(this, "打开目录", "home/laixhe/Desktop");
    //QMessageBox::information(this, "打开目录", "打开目录:" + dirName);

    // 打开一个文件, 得到这个文件的绝对路径
    //QString fileName = QFileDialog::getOpenFileName(this, "打开文件", "home/laixhe/Desktop", "Text files (*.txt)");
    //QMessageBox::information(this, "打开文件", "打开文件:" + fileName);

    // 打开多个文件, 得到这多个文件的绝对路径 QFileDialog::getOpenFileNames
    // 打开一个目录, 使用这个目录来保存指定的文件
    QString fileName = QFileDialog::getSaveFileName(this, "保存文件");
    QMessageBox::information(this, "保存文件", "保存文件:" + fileName);

}
// 选择字体对话框(点击事件)
void MainWindow::on_fontDialogBut_clicked(){
    // QFont 通用参数
    // family: 本地字库中的字体名, 通过 office 等文件软件可以查看
    // pointSize: 字体的字号
    // weight: 字体的粗细, 有效范围为 0 ~ 99
    // italic: 字体是否倾斜显示, 默认不倾斜
    //
    bool ok;
    QFont ft = QFontDialog::getFont(&ok, QFont("微软雅黑", 12, QFont::Bold), this, "选择字体");
    if(ok) {
        ui->fontDialogEdit->setFont(ft);
    }
}
// 颜色选择对话框(点击事件)
void MainWindow::on_colorDialogBut_clicked() {
    // 颜色选择对话框
    QColor color = QColorDialog::getColor();
    // 填充颜色
    QBrush brush(color);
    // 矩形
    QRect rect(0, 0, ui->colorDialogLabel->width(), ui->colorDialogLabel->height());
    QPixmap pix(rect.width(), rect.height());
    // 绘图
    QPainter pain(&pix);
    pain.fillRect(rect, brush);
    //
    ui->colorDialogLabel->setPixmap(pix);
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


#include "test_thread.h"
#include "ui_test_thread.h"

#include <iostream>
#include <QThread>
#include <QDebug>

#include "test_thread_my.h"

// 在 qt 中使用了多线程，有些事项是需要额外注意的：
// - 默认的线程在Qt中称之为窗口线程，也叫主线程，负责窗口事件处理或者窗口控件数据的更新
// - 子线程负责后台的业务逻辑处理，子线程中不能对窗口对象做任何操作，这些事情需要交给窗口线程处理
// - 主线程和子线程之间如果要进行数据的传递，需要使用Qt中的信号槽机制

TestThread::TestThread(QWidget *parent) : QDialog(parent), ui(new Ui::TestThread){
    ui->setupUi(this);

    // 创建程线方法1

    // 创建子线对象
    TestThreadMy *tt = new TestThreadMy(this); // 此处可利用QT自动回收机制

    // 用于在主程线调用 - 循环随机数的数量
    connect(this, &TestThread::starting, tt, &TestThreadMy::recvNum);

    // 启动子线程(开始1的点击事件)
    connect(ui->startBut, &QPushButton::clicked, this, [=](){
        // 发送信号 - 用于子线程 循环随机数的数量
        emit starting(100);
        // 启动子线程
        tt->start();
    });

    // 接收子线程发送的数据
    connect(tt, &TestThreadMy::sendArray, this, [=](QVector<int> data){
        for(int i=0; i<data.size(); ++i){
            ui->randList->addItem(QString::number(data.at(i)));
        }
    });
    

    // 流程
    // 主程线starting -> 子线程recvNum -> 子线程start -> 子线程sendArray -> 主程线(接收子线程发送的数据)

    // 创建程线方法2

    // 1.创建子线对象
    QThread *t2 = new QThread;

    // 2.任务
    TestThreadMy2 *tt2 = new TestThreadMy2; // 此处是线程类不可赋于父对象，无法利用QT自动回收机制，需自已回收内存

    // 3.将任务对象移动到某个子线程中
    tt2->moveToThread(t2);

    // 5.用于在主程线调用 - 程线2的执行任务
    connect(this, &TestThread::starting2, tt2, &TestThreadMy2::working);

    // 4.启动子线程(开始2的点击事件)
    connect(ui->startBut2, &QPushButton::clicked, this, [=](){
        // 发送信号 - 用于子线程 循环随机数的数量
        emit starting2(100);
        // 启动子线程，但未执行任务 working
        t2->start();
    });

    // 6.接收子线程发送的数据
    connect(tt2, &TestThreadMy2::sendArray, this, [=](QVector<int> data){
        for(int i=0; i<data.size(); ++i){
            ui->randList2->addItem(QString::number(data.at(i)));
        }
    });

    // 无法利用QT自动回收机制，又不想定义为成员变量时，可用此处作法
    // 当前窗口(类)在析构时会发出信号 destroyed，可利用这个进行释放资源
    connect(this, &TestThread::destroyed, this, [=]{
        t2->quit();
        t2->wait(); // 等待
        t2->deleteLater(); // 相当于 delete t1;

        tt2->deleteLater(); // 相当于 delete tt2;
    });

}

TestThread::~TestThread(){
    delete ui;
}

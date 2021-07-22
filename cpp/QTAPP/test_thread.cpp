#include "test_thread.h"
#include "ui_test_thread.h"

#include "test_thread_my.h"

// 在 qt 中使用了多线程，有些事项是需要额外注意的：
// - 默认的线程在Qt中称之为窗口线程，也叫主线程，负责窗口事件处理或者窗口控件数据的更新
// - 子线程负责后台的业务逻辑处理，子线程中不能对窗口对象做任何操作，这些事情需要交给窗口线程处理
// - 主线程和子线程之间如果要进行数据的传递，需要使用Qt中的信号槽机制

TestThread::TestThread(QWidget *parent) : QDialog(parent), ui(new Ui::TestThread){
    ui->setupUi(this);

    // 创建子线对象
    TestThreadMy *tt = new TestThreadMy;

    // 用于在主程线调用 - 循环随机数的数量
    connect(this, &TestThread::starting, tt, &TestThreadMy::recvNum);

    // 启动子线程
    connect(ui->startBut, &QPushButton::clicked, this, [=](){

        // 发送信号 - 用于子线程 循环随机数的数量
        emit starting(100);
        // 启动子线程
        tt->start();
    });

    // 接收子线程发送的数据
    connect(tt, &TestThreadMy::sendArray, this, [=](QVector<int> data){

    });
    

    // 流程
    // 主程线starting -> 子线程recvNum -> 子线程start -> 子线程sendArray -> 主程线(接收子线程发送的数据)

}

TestThread::~TestThread(){
    delete ui;
}

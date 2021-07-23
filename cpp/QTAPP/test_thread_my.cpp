#include "test_thread_my.h"

#include <QElapsedTimer>
#include <QRandomGenerator>
#include <QDebug>

// 创建程线方法1
TestThreadMy::TestThreadMy(QObject *parent) : QThread(parent) {
}

// 用于在主程线调用 - 循环随机数的数量
void TestThreadMy::recvNum(int num){
    m_num = num;
}

// 随机数
void TestThreadMy::run(){
    qDebug() << "线程地址：" << QThread::currentThread();

    // 用于存储随机数
    QVector<int> data;

    // 计时器 - 用于计算某段代码的执行耗时
    QElapsedTimer etime;
    // 开始计时
    etime.start();

    for(int i=0; i<m_num; ++i){
        data.push_back( QRandomGenerator::global()->bounded(0, 100000) );
    }

    // 得到耗时操作所花费的具体时间，以毫秒计算
    int milsec = etime.elapsed();
    qDebug() << "线程地址：" << QThread::currentThread() << " 生成" << m_num << "个随机数总共用时：" << milsec << "毫秒";

    // 将随机数通过信号发送出来 - 到主程线
    emit sendArray(data);
}

// 创建程线方法2

// 程线2的执行任务
void TestThreadMy2::working(int num){
    qDebug() << "线程地址：" << QThread::currentThread();

    // 用于存储随机数
    QVector<int> data;

    // 计时器 - 用于计算某段代码的执行耗时
    QElapsedTimer etime;
    // 开始计时
    etime.start();

    for(int i=0; i<num; ++i){
        data.push_back( QRandomGenerator::global()->bounded(0, 100000) );
    }

    // 得到耗时操作所花费的具体时间，以毫秒计算
    int milsec = etime.elapsed();
    qDebug() << "线程地址：" << QThread::currentThread() << " 生成" << num << "个随机数总共用时：" << milsec << "毫秒";

    // 将随机数通过信号发送出来 - 到主程线
    emit sendArray(data);
}

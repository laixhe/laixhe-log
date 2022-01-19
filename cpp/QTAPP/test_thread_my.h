#ifndef TESTTHREADMY_H
#define TESTTHREADMY_H

#include <QObject>
#include <QThread>
#include <QVector>

// 创建程线方法1
class TestThreadMy : public QThread{

    Q_OBJECT
public:
    explicit TestThreadMy(QObject *parent = nullptr);

// 自定义槽
public slots:
    // 用于在主程线调用 - 循环随机数的数量
    void recvNum(int num);

protected:
    // 开始执行线程
    void run() override;

private:
    int m_num = 0;

signals:
    // 将随机数通过信号发送出来 - 到主程线
    void sendArray(QVector<int> data);
};

// 创建程线方法2
class TestThreadMy2 : public QObject {
    Q_OBJECT

public:
    // 程线2的执行任务
    void working(int num); // 函数名可以根据实际需求而定

signals:
    // 将随机数通过信号发送出来 - 到主程线
    void sendArray(QVector<int> data);

};

#endif // TESTTHREADMY_H

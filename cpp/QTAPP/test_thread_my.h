#ifndef TESTTHREADMY_H
#define TESTTHREADMY_H

#include <QThread>
#include <QVector>

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

#endif // TESTTHREADMY_H

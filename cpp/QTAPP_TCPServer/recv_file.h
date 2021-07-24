#ifndef RECVFILE_H
#define RECVFILE_H

#include <QObject>
#include <QThread>
#include <QTcpSocket>

class RecvFile : public QThread {
    Q_OBJECT

public:
    explicit RecvFile(QTcpSocket *client, QObject *parent = nullptr);

signals:

protected:
    // 重写多线程的执行方法
    void run() override;

private:
    QTcpSocket* m_client;

};

#endif // RECVFILE_H

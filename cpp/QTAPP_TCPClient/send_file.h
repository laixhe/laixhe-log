#ifndef SENDFILE_H
#define SENDFILE_H

#include <QObject>
#include <QString>
#include <QByteArray>
#include <QTcpSocket>
#include <QHostAddress>

class SendFile : public QObject {
    Q_OBJECT

public:
    explicit SendFile(QObject *parent = nullptr);

    // 连接服务器
    void connectServer(QString ip, unsigned short port);
    // 发送文件
    void sendFile(QString path);

signals:
    // 连接服务器OK的信号
    void connectOK();
    // 连接断开的信号
    void gamevoer();
    // 当前文件传输进度的信号
    void curPercent(int num);

private:
    QTcpSocket* m_client;
};

#endif // SENDFILE_H

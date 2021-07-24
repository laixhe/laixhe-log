#include "send_file.h"

#include <QFile>
#include <QFileInfo>

SendFile::SendFile(QObject *parent) : QObject(parent) {
}

// 连接服务器
void SendFile::connectServer(QString ip, unsigned short port){
    // 创建通信的套接字对象
    m_client = new QTcpSocket;
    // 连接服务器
    m_client->connectToHost(QHostAddress(ip), port);
    // 连接等待-检测是否和服务器是否连接成功了
    connect(m_client, &QTcpSocket::connected, this, &SendFile::connectOK);
    // 连接等待-检测服务器是否和客户端断开了连接
    connect(m_client, &QTcpSocket::disconnected, this, [=](){
        m_client->close();
        m_client->deleteLater();

        // 发射(发送)-连接断开的信号
        emit gamevoer(); // 发到主线程
    });
}


// 发送文件
void SendFile::sendFile(QString path){

    QFile file(path);
    bool isopen = file.open(QFile::ReadOnly); // 读操作
    if (!isopen) {
        return;
    }

    QFileInfo fInfo(path);
    int fileSize = fInfo.size();

    // 读取文件发给服务端

    int num = 0;
    while (!file.atEnd()) {
        // 首次发送文件大小
        if(num == 0) {
            m_client->write( (char*)&fileSize, 4);
        }

        // 读取每一行
        QByteArray line = file.readLine();
        num += line.size(); // 累记每次发送的长度

        // 发送给服务端
        m_client->write(line);

        // 发射(发送)-当前文件传输进度的信号
        emit curPercent( (num*100/fileSize) ); // 发到主线程
    }
}

#include "recv_file.h"

#include <QString>
#include <QByteArray>
#include <QFile>

RecvFile::RecvFile(QTcpSocket *client, QObject *parent) : QThread(parent){
    m_client = client;
}

// 重写多线程的执行方法
void RecvFile::run(){

    QFile *file = new QFile("./recv.txt");
    bool isopen = file->open(QFile::WriteOnly); // 写操作
    if (!isopen) {
        return;
    }

    // 检测是否有客户端数据的信号 (必须要初始化 m_client)
    connect(m_client, &QTcpSocket::readyRead, this, [=](){
        static int count = 0;
        static int total = 0;

        if(count == 0){
            //
            m_client->read( (char*)&total, 4 );
        }
        // 接收数据
        QByteArray recvMsg = m_client->readAll();
        file->write(recvMsg);
        count += recvMsg.size();

        if(count == total) {
            m_client->close();
            m_client->deleteLater();
            file->close();
            file->deleteLater();
        }

    });

    // 进入事件循环
    exec();
}

#include "main_window.h"
#include "./ui_main_window.h"

#include <QLabel>

// 服务器端
// 通信流程:
// 1. 创建套接字服务器 QTcpServer 对象
// 2. 通过 QTcpServer 对象设置监听，即：QTcpServer::listen()
// 3. 基于 QTcpServer::newConnection() 信号检测是否有新的客户端连接
// 4. 如果有新的客户端连接调用 QTcpSocket *QTcpServer::nextPendingConnection() 得到通信的套接字对象
// 5. 使用通信的套接字对象 QTcpSocket 和客户端进行通信

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);

    // 设置一个默认端口
    ui->port->setText("5050");
    // 设置状态栏
    QLabel *m_status = new QLabel;
    m_status->setPixmap(QPixmap(":/disconnect.png").scaled(20, 20));
    ui->statusbar->addWidget(new QLabel("连接状态：")); // 此处可利用QT自动回收机制(父对象)
    ui->statusbar->addWidget(m_status);               // 此处可利用QT自动回收机制(父对象)

    setWindowTitle("TCP - 服务器");
    // 创建 QTcpServer 对象
    m_server = new QTcpServer(this); // 此处可利用QT自动回收机制(父对象)

    // 检测是否有新的客户端连接的信号
    connect(m_server, &QTcpServer::newConnection, this, [=](){
        // 获得客户端连接
        m_client = m_server->nextPendingConnection();

        ui->record->append("成功和客户端建立了新的连接...");

        m_status->setPixmap(QPixmap(":/connect.png").scaled(20, 20));

        // 检测是否有客户端数据的信号 (必须要初始化 m_client)
        connect(m_client, &QTcpSocket::readyRead, this, [=](){
            // 接收数据
            QString recvMsg = m_client->readAll();

            ui->record->append("客户端: " + recvMsg);
        });

        // 客户端断开了连接的信号 (必须要初始化 m_client)
        connect(m_client, &QTcpSocket::disconnected, this, [=]() {
            ui->record->append("客户端已经断开了连接...");

            m_client->close();
            m_client->deleteLater(); // 相当于 delete m_client

            m_status->setPixmap(QPixmap(":/disconnect.png").scaled(20, 20));
        });

    });
}

MainWindow::~MainWindow() {
    delete ui;
}

// 启动服务按钮
void MainWindow::on_setListen_clicked() {
    // 获取端口
    unsigned short port = ui->port->text().toUShort();
    // 监听服务
    m_server->listen(QHostAddress::Any, port);
    // 将按钮设置为不可用
    ui->setListen->setDisabled(true);
}

// 发送信息按钮
void MainWindow::on_sendMsg_clicked() {
    // 获取发送信息
    QString sendMsg  = ui->msg->toPlainText();
    // 发送
    m_client->write(sendMsg.toUtf8());

    ui->record->append("服务器: " + sendMsg);
    ui->msg->clear();
}


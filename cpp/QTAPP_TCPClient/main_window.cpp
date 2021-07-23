#include "main_window.h"
#include "./ui_main_window.h"

#include <QLabel>

// TCPClient
// 通信流程:
// 1. 创建通信的套接字类 QTcpSocket 对象
// 2. 使用服务器端绑定的 IP 和端口连接服务器 QAbstractSocket::connectToHost()
// 3. 使用 QTcpSocket 对象和服务器进行通信

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);

    // 设置一个默认IP端口
    ui->port->setText("5050");
    ui->ip->setText("127.0.0.1");

    // 设置状态栏
    QLabel *m_status = new QLabel;
    m_status->setPixmap(QPixmap(":/disconnect.png").scaled(20, 20));
    ui->statusbar->addWidget(new QLabel("连接状态：")); // 此处可利用QT自动回收机制(父对象)
    ui->statusbar->addWidget(m_status);               // 此处可利用QT自动回收机制(父对象)

    setWindowTitle("TCP - 客户端");

    // 创建通信的套接字对象
    m_client = new QTcpSocket(this);
    // 检测服务器是否回复了数据
    connect(m_client, &QTcpSocket::readyRead, [=](){
        // 接收服务器发送的数据
        QByteArray recvMsg = m_client->readAll();
        ui->record->append("服务器Say: " + recvMsg);
    });

    // 检测是否和服务器是否连接成功了
    connect(m_client, &QTcpSocket::connected, this, [=](){
        ui->record->append("恭喜, 连接服务器成功!!!");
        m_status->setPixmap(QPixmap(":/connect.png").scaled(20, 20));
    });

    // 检测服务器是否和客户端断开了连接
    connect(m_client, &QTcpSocket::disconnected, this, [=](){
        m_status->setPixmap(QPixmap(":/disconnect.png").scaled(20, 20));
        ui->record->append("服务器已经断开了连接, ...");
        ui->connectServer->setEnabled(true);
        ui->disconnect->setEnabled(false);
    });
}

MainWindow::~MainWindow() {
    delete ui;
}

// 连接服务器按钮按下之后的处理动作
void MainWindow::on_connectServer_clicked(){
    QString ip = ui->ip->text();
    unsigned short port = ui->port->text().toInt();
    // 连接服务器
    m_client->connectToHost(QHostAddress(ip), port);
    ui->connectServer->setEnabled(false);
    ui->disconnect->setEnabled(true);
}

// 发送数据按钮按下之后的处理动作
void MainWindow::on_sendMsg_clicked(){
    QString sendMsg = ui->msg->toPlainText();
    m_client->write(sendMsg.toUtf8());
    ui->record->append("客户端Say: " + sendMsg);
    ui->msg->clear();
}

// 断开连接按钮被按下之后的处理动作
void MainWindow::on_disconnect_clicked(){
    m_client->close();
    ui->connectServer->setEnabled(true);
    ui->disconnect->setEnabled(false);
}


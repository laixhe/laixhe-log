#include "main_window.h"
#include "./ui_main_window.h"

#include <QThread>
#include <QMessageBox>
#include <QLabel>
#include <QProgressBar>
#include <QFileDialog>

#include "send_file.h"

// TCPClient
// 通信流程:
// 1. 创建通信的套接字类 QTcpSocket 对象
// 2. 使用服务器端绑定的 IP 和端口连接服务器 QAbstractSocket::connectToHost()
// 3. 使用 QTcpSocket 对象和服务器进行通信

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);

    // 设置进度条
    ui->progressBar->setRange(0, 100);
    ui->progressBar->setValue(0);
    // 设置一个默认IP端口
    ui->port->setText("5050");
    ui->ip->setText("127.0.0.1");
    // 设置状态栏
    QLabel *m_status = new QLabel;
    m_status->setPixmap(QPixmap(":/disconnect.png").scaled(20, 20));
    ui->statusbar->addWidget(new QLabel("连接状态：")); // 此处可利用QT自动回收机制(父对象)
    ui->statusbar->addWidget(m_status);               // 此处可利用QT自动回收机制(父对象)
    // 设置标题
    setWindowTitle("TCP - 客户端");

    //------ 文件发送的 ----------

    // 创建线程对象
    QThread *tF = new QThread;
    // 创建任务对象(工作的类对象)
    SendFile *workerF = new SendFile;
    // 将工作的类对象移动到创建的子线程对象中
    workerF->moveToThread(tF);

    // 连接等待-发射(发送)过来的启动文件客户端的信号 (从主线程 -> 子线程)
    connect(this, &MainWindow::startFileConnect, workerF, &SendFile::connectServer);

    // 连接等待-发射(发送)连接服务器OK的信号 (子线程 -> 从主线程)
    connect(workerF, &SendFile::connectOK, this, [=](){
        QMessageBox::information(this, "连接服务器", "成功连接到了服务器...");
    });

    // 连接等待-发射(发送)发送文件的信号 (从主线程 -> 子线程)
    connect(this, &MainWindow::sendFile, workerF, &SendFile::sendFile);

    // 连接等待-发射(发送)连接断开的信号 (子线程 -> 从主线程)
    connect(workerF, &SendFile::gamevoer, this, [=](){
        QMessageBox::information(this, "连接服务器", "断开连接服务器...");
        // 资源释放
        tF->quit();
        tF->wait();
        workerF->deleteLater();
        tF->deleteLater();
    });

    // 连接等待-发射(发送)当前文件传输进度的信号(子线程 -> 从主线程)
    connect(workerF, &SendFile::curPercent, ui->progressBar, &QProgressBar::setValue);

    // 启动线程
    tF->start();

    //------ 信息发送的 ----------

    // 创建通信的套接字对象
    m_client = new QTcpSocket(this);

    // 检测是否和服务器是否连接成功了
    connect(m_client, &QTcpSocket::connected, this, [=](){
        ui->record->append("恭喜, 连接服务器成功!!!");
        m_status->setPixmap(QPixmap(":/connect.png").scaled(20, 20));
    });

    // 检测服务器是否回复了数据
    connect(m_client, &QTcpSocket::readyRead, [=](){
        // 接收服务器发送的数据
        QByteArray recvMsg = m_client->readAll();
        ui->record->append("服务器: " + recvMsg);
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
    ui->record->append("客户端: " + sendMsg);
    ui->msg->clear();
}

// 断开连接按钮被按下之后的处理动作
void MainWindow::on_disconnect_clicked(){
    m_client->close();
    ui->connectServer->setEnabled(true);
    ui->disconnect->setEnabled(false);
}

// ------------------

// 连接文件服务器钮按下之后的处理动作
void MainWindow::on_connectFileServer_clicked(){
    QString ip = ui->ip->text();
    unsigned short port = ui->port->text().toInt();
    // 发射(发送)启动文件客户端的信号
    emit startFileConnect(ip, port); // 发到子线程
}

// 选择文件
void MainWindow::on_selFile_clicked() {
    QString path = QFileDialog::getOpenFileName();
    ui->filePath->setText(path);
}

// 发送文件
void MainWindow::on_sendFile_clicked(){
    // 发射(发送)发送文件的信号
    emit sendFile(ui->filePath->text()); // 发到子线程
}


#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QString>
#include <QMainWindow>
#include <QTcpSocket>
#include <QHostAddress>

QT_BEGIN_NAMESPACE
namespace Ui {
    class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

signals:
    // 启动文件客户端的信号
    void startFileConnect(QString ip, unsigned short port);
    // 发送文件的信号
    void sendFile(QString path);

private slots:
    void on_connectServer_clicked();
    void on_disconnect_clicked();
    void on_sendMsg_clicked();

    void on_connectFileServer_clicked();
    void on_selFile_clicked();
    void on_sendFile_clicked();

private:
    Ui::MainWindow *ui;

    QTcpSocket* m_client;

};
#endif // MAINWINDOW_H

#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QTcpSocket>
#include <QTcpServer>

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

private slots:
    void on_setListen_clicked();

    void on_sendMsg_clicked();

    void on_setFile_clicked();

private:
    Ui::MainWindow *ui;

    QTcpServer* m_server;
    QTcpSocket* m_client;

    QTcpServer* m_file_server;

};
#endif // MAINWINDOW_H

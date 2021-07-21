#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>

#include "signalsfrom.h"
#include "signalsto.h"

QT_BEGIN_NAMESPACE
namespace Ui {
    // 对应的是 UI 文件中的类 <class>MainWindow</class>
    class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void on_signalsFromTo_clicked();
    void on_testDialogBut_clicked();
    void on_messageDialogBut_clicked();
    void on_fileDialogBut_clicked();
    void on_fontDialogBut_clicked();
    void on_colorDialogBut_clicked();
    void on_loginDialogBut_clicked();
    void on_testBut_clicked();

private:
    Ui::MainWindow *ui;

    SignalsFrom *m_signalsFrom;
    SignalsTo *m_signalsTo;

};

#endif // MAINWINDOW_H

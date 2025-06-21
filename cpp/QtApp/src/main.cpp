#include "mainwindow/mainwindow.h"

#include <QApplication>

// main 程序入口 argc 命令行变量数量  argv 命令行变量数组
int main(int argc, char *argv[])
{
    // 应用程序对象,在 Qt 中，应用程序对象 有且仅有一个
    QApplication a(argc, argv);

    MainWindow w;
    // 窗口对象 默认不会显示，必须要调用show方法显示窗口
    w.show();

    // 让应用程序对象进入消息循环(让代码阻塞到这行)
    return a.exec();
}

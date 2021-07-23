#include "logindialog.h"
#include "ui_logindialog.h"

#include <QDebug>

LoginDialog::LoginDialog(QWidget *parent) : QDialog(parent), ui(new Ui::LoginDialog){
    ui->setupUi(this);

    // 设置窗口的固定尺寸
    setFixedSize(330, 330);
    // 隐藏标题栏
    //setWindowFlags(Qt::CustomizeWindowHint);
}

LoginDialog::~LoginDialog() {
    delete ui;
}

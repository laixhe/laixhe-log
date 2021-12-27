#include "testdialog.h"
#include "ui_testdialog.h"

#include <QDebug>

// QDialog
// 对话框窗口类
// 模态(show)和非模态(exec)两种显示方式
// 不能内嵌
// 是 QWidget 类的子类, 处理继承自父类的属性之外, 还有一些自己所特有的属性
//

TestDialog::TestDialog(QWidget *parent) : QDialog(parent), ui(new Ui::TestDialog) {
    ui->setupUi(this);
}

TestDialog::~TestDialog() {
    delete ui;
}

// 关闭当前对话框
void TestDialog::on_acceptBut_clicked(){
    this->accept();
}
// 关闭当前对话框
void TestDialog::on_rejectBut_clicked(){
    this->reject();
}
// 关闭当前对话框
void TestDialog::on_DoneBut_clicked(){
    this->done(2);
}


#include "widget.h"
#include "./ui_widget.h"

#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlQuery>

#include <iostream>

Widget::Widget(QWidget *parent)
    : QWidget(parent)
    , ui(new Ui::Widget)
{
    ui->setupUi(this);

    QSqlDatabase db = QSqlDatabase::addDatabase("QMYSQL");
    db.setHostName("192.168.10.240");
    db.setDatabaseName("byx");
    db.setUserName("root");
    db.setPassword("123456");
    bool ok = db.open();
    if(!ok){
        std::cout << "db open error..." << std::endl;
        return;
    }

    QSqlQuery query("select `id`,`log` from `migrate_db`", db);
    while (query.next()) {
        QString id = query.value(0).toString();
        QString log = query.value(1).toString();

        std::cout << id.toStdString() << ":" << log.toStdString() << std::endl;
    }
    db.close();

}

Widget::~Widget()
{
    delete ui;
}


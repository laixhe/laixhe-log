#ifndef SIGNALSFROM_H
#define SIGNALSFROM_H

#include <QObject>

class SignalsFrom : public QObject {
    Q_OBJECT

public:
    explicit SignalsFrom(QObject *parent = nullptr);

// 自定义信号
signals:
    void testFrom();
};

#endif // SIGNALSFROM_H

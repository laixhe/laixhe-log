#ifndef SIGNALSTO_H
#define SIGNALSTO_H

#include <QObject>

class SignalsTo : public QObject {
    Q_OBJECT

public:
    explicit SignalsTo(QObject *parent = nullptr);

// 自定义槽
public slots:
    void testTo();

};

#endif // SIGNALSTO_H

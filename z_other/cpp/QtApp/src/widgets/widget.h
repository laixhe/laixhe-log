#ifndef QTAPP_WIDGET_H
#define QTAPP_WIDGET_H

#include <QDebug>
#include <QWidget>

class Widget : public QWidget {
Q_OBJECT

public:
    explicit Widget(QWidget *parent = nullptr);
    ~Widget();

};


#endif //QTAPP_WIDGET_H

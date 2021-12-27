#ifndef TESTWIDGET_H
#define TESTWIDGET_H

#include <QWidget>

namespace Ui {
    // 对应的是 UI 文件中的类 <class>TestWidget</class>
    class TestWidget;
}

class TestWidget : public QWidget {
    Q_OBJECT

public:
    explicit TestWidget(QWidget *parent = nullptr);
    ~TestWidget();

private:
    Ui::TestWidget *ui;

};

#endif // TESTWIDGET_H

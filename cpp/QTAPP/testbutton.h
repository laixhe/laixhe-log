#ifndef TESTBUTTON_H
#define TESTBUTTON_H

#include <QDialog>

namespace Ui {
    class TestButton;
}

class TestButton : public QDialog{
    Q_OBJECT

public:
    explicit TestButton(QWidget *parent = nullptr);
    ~TestButton();

private:
    Ui::TestButton *ui;

};

#endif // TESTBUTTON_H

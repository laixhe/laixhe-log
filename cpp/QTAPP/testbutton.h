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

private slots:
    void on_radioCppBut_clicked();
    void on_radioGoBut_clicked();
    void on_radioPhpBut_clicked();
    void on_radioJavaBut_clicked();
    void on_radioPyBut_clicked();
    void on_radioRustBut_clicked();
    void on_cppCheckBox_stateChanged(int arg1);
    void on_goCheckBox_stateChanged(int arg1);
    void on_phpCheckBox_stateChanged(int arg1);
    void on_selectBut_clicked();

private:
    Ui::TestButton *ui;

};

#endif // TESTBUTTON_H

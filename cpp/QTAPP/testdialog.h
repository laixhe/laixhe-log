#ifndef TESTDIALOG_H
#define TESTDIALOG_H

#include <QDialog>

namespace Ui {
    class TestDialog;
}

class TestDialog : public QDialog {
    Q_OBJECT

public:
    explicit TestDialog(QWidget *parent = nullptr);
    ~TestDialog();

private slots:
    void on_acceptBut_clicked();

    void on_rejectBut_clicked();

    void on_DoneBut_clicked();

private:
    Ui::TestDialog *ui;

};

#endif // TESTDIALOG_H

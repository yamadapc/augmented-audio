//
// Created by Pedro Tacla Yamada on 10/11/21.
//

#pragma once

#include <QDockWidget>
#include <QMainWindow>
#include <QPushButton>
#include <QtCore/qfile.h>

QT_BEGIN_NAMESPACE
namespace Ui
{
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit MainWindow (QWidget* parent = nullptr);

    ~MainWindow () override;

private:
    Ui::MainWindow* ui;

    QDockWidget* leftDockWidget;
    QWidget* contentsWindow;
    QPushButton* button;
};

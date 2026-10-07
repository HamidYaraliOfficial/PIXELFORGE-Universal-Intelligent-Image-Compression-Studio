#include <QApplication>
#include "mainwindow.h"

int main(int argc, char** argv) {
    QApplication app(argc, argv);
    app.setApplicationName("PIXELFORGE");
    app.setOrganizationName("PIXELFORGE");
    MainWindow w;
    w.show();
    return app.exec();
}

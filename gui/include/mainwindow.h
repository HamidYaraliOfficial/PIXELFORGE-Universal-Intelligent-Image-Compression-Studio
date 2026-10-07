#pragma once
#include <QMainWindow>
#include <QStackedWidget>
#include <QListWidget>
#include <QLabel>
#include <QProgressBar>
#include <QComboBox>
#include <QLineEdit>
#include <QSpinBox>
#include <QTextEdit>
#include <QSplitter>
#include "backendclient.h"

class MainWindow : public QMainWindow {
    Q_OBJECT
public:
    explicit MainWindow(QWidget* parent=nullptr);
private:
    BackendClient backend_;
    QStackedWidget* pages_;
    QListWidget* navigation_;
    QLabel* statusLabel_;
    QLabel* statsLabel_;
    QListWidget* queue_;
    QComboBox* theme_;
    QComboBox* language_;
    QComboBox* direction_;
    QLineEdit* outputEdit_;
    QSpinBox* qualityMin_;
    QSpinBox* qualityMax_;
    QTextEdit* scheduleEdit_;
    void buildUi();
    void buildDashboard();
    void buildWorkspace();
    void buildSettings();
    void applyTheme(const QString& theme);
    void applyLanguage(const QString& lang);
    void addFiles();
    void addFolder();
    QString selectedOutput(const QString& input) const;
private slots:
    void onJson(const QJsonObject& obj);
};

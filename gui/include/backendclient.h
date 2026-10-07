#pragma once
#include <QObject>
#include <QProcess>
#include <QJsonObject>
#include <functional>

class BackendClient : public QObject {
    Q_OBJECT
public:
    explicit BackendClient(QObject* parent=nullptr);
    void setExecutable(const QString& path);
    void analyze(const QString& file);
    void autoCandidates(const QString& file);
    void compress(const QString& file, const QString& output);
    void scheduleEstimate(const QString& config, int seconds);
signals:
    void jsonReady(const QJsonObject& object);
    void errorMessage(const QString& message);
private slots:
    void onReadyRead();
    void onFinished(int code, QProcess::ExitStatus status);
private:
    QProcess process_;
    QString buffer_;
    QString executable_;
    void run(const QStringList& args);
};

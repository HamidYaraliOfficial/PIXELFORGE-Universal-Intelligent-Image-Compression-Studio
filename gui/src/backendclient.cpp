#include "backendclient.h"
#include <QJsonDocument>
#include <QDir>

BackendClient::BackendClient(QObject* parent):QObject(parent) {
    connect(&process_, &QProcess::readyReadStandardOutput, this, &BackendClient::onReadyRead);
    connect(&process_, &QProcess::readyReadStandardError, this, [this](){
        const auto e = QString::fromLocal8Bit(process_.readAllStandardError());
        if(!e.trimmed().isEmpty()) emit errorMessage(e.trimmed());
    });
    connect(&process_, qOverload<int,QProcess::ExitStatus>(&QProcess::finished), this, &BackendClient::onFinished);
}

void BackendClient::setExecutable(const QString& path) { executable_ = path; }

void BackendClient::run(const QStringList& args) {
    if(executable_.isEmpty()) {
#ifdef Q_OS_WIN
        executable_ = "pixelforge.exe";
#else
        executable_ = "pixelforge";
#endif
    }
    if(process_.state()!=QProcess::NotRunning) process_.kill();
    buffer_.clear();
    process_.start(executable_, args);
    if(!process_.waitForStarted(1200)) emit errorMessage("Unable to start PIXELFORGE engine.");
}

void BackendClient::analyze(const QString& file) { run({"analyze", file}); }
void BackendClient::autoCandidates(const QString& file) { run({"auto", file}); }
void BackendClient::compress(const QString& file, const QString& output) { run({"compress", file, "-o", output}); }
void BackendClient::scheduleEstimate(const QString& config, int seconds) { run({"schedule","estimate","--config",config,"--job-seconds",QString::number(seconds)}); }

void BackendClient::onReadyRead() {
    buffer_ += QString::fromUtf8(process_.readAllStandardOutput());
    int pos=-1;
    while((pos=buffer_.indexOf('\n'))>=0) {
        const QString line=buffer_.left(pos).trimmed();
        buffer_.remove(0,pos+1);
        if(line.isEmpty()) continue;
        QJsonParseError err{};
        const auto doc=QJsonDocument::fromJson(line.toUtf8(), &err);
        if(err.error==QJsonParseError::NoError && doc.isObject()) emit jsonReady(doc.object());
        else emit errorMessage("Engine returned non-JSON output: "+line.left(300));
    }
}
void BackendClient::onFinished(int code, QProcess::ExitStatus status) {
    if(code!=0 || status!=QProcess::NormalExit) emit errorMessage("Engine process finished with an error.");
}

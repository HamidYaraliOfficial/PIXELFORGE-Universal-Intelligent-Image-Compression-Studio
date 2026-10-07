#include "mainwindow.h"
#include <QApplication>
#include <QFileDialog>
#include <QFileInfo>
#include <QDateTime>
#include <QFormLayout>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QVBoxLayout>
#include <QPushButton>
#include <QFrame>
#include <QMessageBox>
#include <QJsonDocument>
#include <QJsonObject>
#include <QRegularExpression>

static QFrame* card() {
    auto* f=new QFrame; f->setObjectName("card");
    f->setFrameShape(QFrame::StyledPanel);
    return f;
}

MainWindow::MainWindow(QWidget* parent):QMainWindow(parent) {
    backend_.setParent(this);
    connect(&backend_, &BackendClient::jsonReady, this, &MainWindow::onJson);
    connect(&backend_, &BackendClient::errorMessage, this, [this](const QString& m){statusLabel_->setText(m);});
    buildUi();
}

void MainWindow::buildUi() {
    setWindowTitle("PIXELFORGE — Universal Intelligent Image Compression Studio");
    resize(1440, 900);
    auto* root=new QWidget; auto* layout=new QHBoxLayout(root);
    navigation_=new QListWidget;
    navigation_->addItems({"Dashboard","Compression Workspace","Batch Manager","Format Explorer","Presets","Reports","History","Settings"});
    navigation_->setFixedWidth(220);
    layout->addWidget(navigation_);
    pages_=new QStackedWidget; layout->addWidget(pages_,1);
    buildDashboard(); buildWorkspace(); buildSettings();
    pages_->addWidget(new QWidget); pages_->addWidget(new QWidget); pages_->addWidget(new QWidget); pages_->addWidget(new QWidget); pages_->addWidget(new QWidget);
    connect(navigation_, &QListWidget::currentRowChanged, pages_, &QStackedWidget::setCurrentIndex);
    navigation_->setCurrentRow(0);
    statusLabel_=new QLabel("Ready");
    statusBar()->addWidget(statusLabel_,1);
    setCentralWidget(root);
    applyTheme("Windows Default");
}

void MainWindow::buildDashboard() {
    auto* page=new QWidget; auto* v=new QVBoxLayout(page);
    auto* title=new QLabel("PIXELFORGE Dashboard"); title->setObjectName("title"); v->addWidget(title);
    auto* grid=new QGridLayout;
    const QStringList labels={"Images","Original Size","Output Size","Saved","Avg. Reduction","Avg. SSIM","Processing Time","Active Workers"};
    for(int i=0;i<labels.size();++i){
        auto* c=card(); auto* l=new QVBoxLayout(c);
        l->addWidget(new QLabel(labels[i]));
        auto* value=new QLabel("0"); value->setObjectName("stat");
        l->addWidget(value); grid->addWidget(c,i/4,i%4);
    }
    v->addLayout(grid);
    statsLabel_=new QLabel("Automatic Decision Engine ready."); v->addWidget(statsLabel_);
    auto* bar=new QProgressBar; bar->setRange(0,100); bar->setValue(0); v->addWidget(bar);
    queue_=new QListWidget; v->addWidget(queue_,1);
    auto* row=new QHBoxLayout;
    auto* add=new QPushButton("Add Images"); auto* folder=new QPushButton("Add Folder");
    connect(add,&QPushButton::clicked,this,&MainWindow::addFiles);
    connect(folder,&QPushButton::clicked,this,&MainWindow::addFolder);
    row->addWidget(add); row->addWidget(folder); row->addStretch(); v->addLayout(row);
    pages_->addWidget(page);
}

void MainWindow::buildWorkspace() {
    auto* page=new QWidget; auto* v=new QVBoxLayout(page);
    auto* title=new QLabel("Compression Workspace"); title->setObjectName("title"); v->addWidget(title);
    auto* split=new QSplitter(Qt::Horizontal);
    auto* left=card(); auto* lv=new QVBoxLayout(left);
    auto* input=new QPushButton("Choose Image"); lv->addWidget(input);
    auto* preview=new QLabel("Drop an image here"); preview->setAlignment(Qt::AlignCenter); preview->setMinimumSize(480,360); lv->addWidget(preview,1);
    auto* right=card(); auto* rv=new QVBoxLayout(right);
    auto* form=new QFormLayout;
    auto* output=new QComboBox; output->addItems({"Automatic","JPEG","PNG","WebP","AVIF","TIFF","BMP","GIF","PNM"});
    auto* goal=new QComboBox; goal->addItems({"Balanced","Maximum Compression","Maximum Quality","Web Optimized"});
    qualityMin_=new QSpinBox; qualityMin_->setRange(1,100); qualityMin_->setValue(60);
    qualityMax_=new QSpinBox; qualityMax_->setRange(1,100); qualityMax_->setValue(92);
    outputEdit_=new QLineEdit; outputEdit_->setPlaceholderText("Output file");
    form->addRow("Output Format",output); form->addRow("Goal",goal); form->addRow("Min Quality",qualityMin_); form->addRow("Max Quality",qualityMax_); form->addRow("Output",outputEdit_);
    rv->addLayout(form);
    auto* analyze=new QPushButton("Analyze & Auto-Select"); rv->addWidget(analyze);
    auto* compress=new QPushButton("Run Compression"); rv->addWidget(compress);
    auto* metrics=new QTextEdit; metrics->setReadOnly(true); rv->addWidget(metrics,1);
    split->addWidget(left); split->addWidget(right); split->setSizes({800,520}); v->addWidget(split,1);
    connect(input,&QPushButton::clicked,this,[this,preview](){
        const auto f=QFileDialog::getOpenFileName(this,"Open Image",QString(), "Images (*.jpg *.jpeg *.png *.webp *.avif *.tif *.tiff *.bmp *.gif *.ppm *.pgm)");
        if(f.isEmpty()) return; preview->setText(QFileInfo(f).fileName()); outputEdit_->setText(selectedOutput(f)); backend_.analyze(f);
    });
    pages_->addWidget(page);
}

void MainWindow::buildSettings() {
    auto* page=new QWidget; auto* v=new QVBoxLayout(page);
    auto* title=new QLabel("Settings"); title->setObjectName("title"); v->addWidget(title);
    theme_=new QComboBox; theme_->addItems({"Windows Default","Light","Dark","Blue","Red","High Contrast","AMOLED"});
    language_=new QComboBox; language_->addItems({"English","فارسی","中文"});
    direction_=new QComboBox; direction_->addItems({"Auto","LTR","RTL"});
    auto* form=new QFormLayout; form->addRow("Theme",theme_); form->addRow("Language",language_); form->addRow("Text Direction",direction_);
    v->addLayout(form);
    scheduleEdit_=new QTextEdit; scheduleEdit_->setPlaceholderText(R"({"timezone":"local","windows":[{"weekday":0,"start":"09:00","end":"17:00"}],"queue_jobs_only_inside_windows":true})");
    v->addWidget(new QLabel("Processing Hours / Schedule Windows")); v->addWidget(scheduleEdit_,1);
    auto* estimate=new QPushButton("Estimate Next Window / ETA"); v->addWidget(estimate);
    connect(theme_, &QComboBox::currentTextChanged, this, &MainWindow::applyTheme);
    connect(language_, &QComboBox::currentTextChanged, this, &MainWindow::applyLanguage);
    connect(direction_, &QComboBox::currentTextChanged, this, [this](const QString& d){
        if(d=="RTL") qApp->setLayoutDirection(Qt::RightToLeft);
        else if(d=="LTR") qApp->setLayoutDirection(Qt::LeftToRight);
        else applyLanguage(language_->currentText());
    });
    connect(estimate,&QPushButton::clicked,this,[this](){
        const QString path=QDir::tempPath()+"/pixelforge-schedule.json";
        QFile f(path); if(f.open(QIODevice::WriteOnly)){ f.write(scheduleEdit_->toPlainText().toUtf8()); f.close(); backend_.scheduleEstimate(path,600); }
    });
    pages_->addWidget(page);
}

void MainWindow::applyTheme(const QString& theme) {
    QString qss=R"(
        QMainWindow,QWidget{font-family:"Segoe UI";font-size:13px;}
        QListWidget{border:0;padding:8px;}
        QListWidget::item{padding:12px;border-radius:8px;}
        QFrame#card{border:1px solid palette(mid);border-radius:14px;padding:8px;}
        QLabel#title{font-size:26px;font-weight:700;margin:8px;}
        QLabel#stat{font-size:24px;font-weight:700;}
        QPushButton{padding:9px 14px;border-radius:8px;}
    )";
    if(theme=="Dark" || theme=="AMOLED") qss += "QMainWindow,QWidget{background:#111; color:#eee;} QFrame#card{background:#191919;}";
    else if(theme=="Blue") qss += "QPushButton{background:#2563eb;color:white;} QFrame#card{background:#f4f8ff;}";
    else if(theme=="Red") qss += "QPushButton{background:#b91c1c;color:white;} QFrame#card{background:#fff5f5;}";
    else if(theme=="High Contrast") qss += "QWidget{background:black;color:white;} QPushButton{background:white;color:black;}";
    setStyleSheet(qss);
}

void MainWindow::applyLanguage(const QString& lang) {
    if(lang=="فارسی") qApp->setLayoutDirection(Qt::RightToLeft);
    else qApp->setLayoutDirection(Qt::LeftToRight);
}

void MainWindow::addFiles() {
    const auto fs=QFileDialog::getOpenFileNames(this,"Add Images",QString(),"Images (*.jpg *.jpeg *.png *.webp *.avif *.tif *.tiff *.bmp *.gif *.ppm *.pgm)");
    for(const auto& f:fs) queue_->addItem(f);
}

void MainWindow::addFolder() {
    const auto d=QFileDialog::getExistingDirectory(this,"Add Folder");
    if(!d.isEmpty()) queue_->addItem(d+"  [recursive batch]");
}

QString MainWindow::selectedOutput(const QString& input) const {
    QFileInfo info(input);
    return info.path()+"/"+info.completeBaseName()+"-optimized.webp";
}

void MainWindow::onJson(const QJsonObject& obj) {
    statsLabel_->setText(QString::fromUtf8(QJsonDocument(obj).toJson(QJsonDocument::Indented)));
    statusLabel_->setText("Engine response received");
}

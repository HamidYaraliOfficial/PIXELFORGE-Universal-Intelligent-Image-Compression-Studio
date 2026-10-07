# PIXELFORGE — Universal Intelligent Image Compression Studio

## English

### Overview
PIXELFORGE is a native-oriented desktop and command-line image optimization platform built around a Rust processing engine and a Qt 6 interface. The project is organized as a shared engine so the GUI, CLI and future automation clients operate on the same pipeline and JSON contract.

### Core capabilities
- Content-aware image analysis before compression.
- Automatic candidate generation with Fast Auto, Balanced Auto and Deep Auto.
- Lossless, visually-lossless and lossy operating modes.
- Multi-objective selection using size, reduction, perceptual quality and target constraints.
- Quality Guardian verification after encoding.
- Single-file and recursive batch processing.
- Parallel batch execution with per-file isolation.
- Target file-size and target-reduction modes.
- JPEG, PNG, WebP, AVIF, TIFF, BMP, GIF and PNM through the baseline Rust image backend.
- HEIF/HEIC and JPEG XL extension points for optional native backends.
- Metadata inspection and privacy-oriented policy model.
- Duplicate detection primitives through cryptographic hash, dHash and aHash.
- SQLite/WAL persistence foundation.
- JSON, CSV and HTML reporting.
- Schedule windows and next-window/ETA calculation.
- Windows/Linux setup and build scripts.
- Qt 6 Windows-11-inspired desktop shell with light, dark, Windows Default, blue, red and high-contrast themes.
- English, Persian and Chinese language direction handling.
- Versioned JSON-line engine communication for GUI/automation separation.

### Repository layout
```text
PIXELFORGE/
├─ core/
│  ├─ src/
│  │  ├─ analysis/
│  │  ├─ compression/
│  │  ├─ database/
│  │  ├─ metadata/
│  │  ├─ optimizer/
│  │  ├─ pipeline/
│  │  ├─ quality/
│  │  ├─ report/
│  │  ├─ rules/
│  │  ├─ scheduler/
│  │  └─ util/
│  └─ tests/
├─ gui/
├─ scripts/
├─ config/
├─ docs/
├─ tests/
└─ .github/workflows/
```

### Quick start — Windows PowerShell
```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\scripts\setup-windows.ps1
.\scripts\build.ps1
.\scripts\test.ps1
.\target\release\pixelforge.exe analyze .\photo.jpg
.\target\release\pixelforge.exe auto .\photo.jpg
.\target\release\pixelforge.exe compress .\photo.jpg -o .\optimized.webp
```

### Quick start — Linux Bash
```bash
chmod +x scripts/*.sh
./scripts/setup-linux.sh
./scripts/build.sh
./scripts/test.sh
./target/release/pixelforge analyze ./photo.jpg
./target/release/pixelforge auto ./photo.jpg
./target/release/pixelforge compress ./photo.jpg -o ./optimized.webp
```

### CLI
```text
pixelforge compress image.jpg -o output.webp
pixelforge auto image.jpg
pixelforge batch ./images -o ./optimized
pixelforge analyze image.jpg
pixelforge benchmark image.jpg --depth deep
pixelforge convert image.png -o output.webp --auto
pixelforge target image.jpg --size 500KB -o output.webp
pixelforge report runs.json --format html -o report.html
pixelforge schedule estimate --config config/default.json --job-seconds 600
```

### Automatic compression
The analysis engine extracts bounded statistical and structural features. Candidate generation then combines content class, alpha presence, texture complexity, screenshot/text density and the configured user target. Each candidate is encoded and decoded for verification. The selection objective changes with the requested goal.

### Schedule and time windows
The configuration accepts weekday windows using ISO-style weekday numbers:
- 0 = Monday
- 1 = Tuesday
- ...
- 6 = Sunday

Example:
```json
{
  "timezone": "local",
  "windows": [
    {"weekday":0,"start":"09:00","end":"17:00"},
    {"weekday":2,"start":"14:00","end":"22:30"}
  ],
  "queue_jobs_only_inside_windows": true
}
```
The CLI reports whether the current local time is inside a configured window, the remaining seconds in the current window, the next opening window and the seconds until it begins. A job-duration input is included so an operator can compare ETA to remaining availability.

### Quality and reports
Each run returns original size, final size, saved bytes, reduction percentage, selected candidate, quality metrics, processing time, decoder verification and selection reason. Batch reports can be exported to JSON, CSV or HTML.

### GUI
The Qt 6 shell provides Dashboard, Compression Workspace and Settings as a stable native foundation. The Workspace exposes output format, goal and quality range. Settings expose theme, language direction and operating-hour schedule JSON. Heavy compression remains in the Rust process rather than the Qt event loop.

### Theming and localization
- Windows Default
- Light
- Dark
- Blue
- Red
- High Contrast
- AMOLED-style dark mode

English and Chinese use LTR. Persian uses RTL. The application also allows explicit LTR/RTL override from Settings.

### Architecture diagram
```text
Qt 6 GUI / CLI / Automation
            │
            ▼
     Versioned JSON API
            │
            ▼
      Rust Core Pipeline
 ┌──────────┼────────────────────────────────┐
 ▼          ▼                                ▼
Validate   Analyze                      Scheduler
 ▼          ▼                                ▼
Decode → Candidate Generator → Encode → Verify
                         │             │
                         ▼             ▼
                    Quality Engine   Reports
                         │
                         ▼
                       SQLite
```

### Native codec extension
The baseline Rust backend is intentionally buildable without forcing proprietary or platform-specific codec SDKs. The `docs/PLUGIN_SDK.md` file defines the extension boundary for native implementations. Deployment-specific builds can supply native JPEG XL and HEIF/HEIC adapters using C ABI libraries or sandboxed helper processes.

### Testing
- Unit tests for optimizer and schedule structures.
- Integration-ready core test target.
- Corruption and golden-image fixtures directory.
- CI for Windows and Linux Rust builds.
- Linux Qt build workflow.
- The fixture directory is intentionally kept data-light; add licensed reference images required for your organization.

### Troubleshooting
See `docs/INSTALLATION_GUIDE.md` for Windows and Linux commands, Qt discovery, shared-library deployment and codec backend notes.

### License
MIT.

---

# فارسی

## معرفی
PIXELFORGE یک استودیو حرفه‌ای فشرده‌سازی و بهینه‌سازی تصویر است که هسته پردازش آن با Rust و رابط گرافیکی آن با Qt 6/C++ طراحی شده است. معماری پروژه طوری تنظیم شده که GUI و CLI هر دو از یک موتور پردازش مشترک و یک قرارداد JSON استفاده کنند.

## قابلیت‌ها
- تحلیل تصویر قبل از فشرده‌سازی.
- انتخاب خودکار با Fast Auto، Balanced Auto و Deep Auto.
- حالت Lossless، Visually Lossless و Lossy.
- انتخاب چندمعیاره بر پایه حجم، درصد کاهش، کیفیت ادراکی و هدف کاربر.
- Quality Guardian برای بررسی خروجی پس از فشرده‌سازی.
- پردازش تکی و Batch با پوشه و پوشه‌های تو در تو.
- پردازش موازی و مدیریت خطای جداگانه برای هر فایل.
- Target Size و Target Reduction.
- پشتیبانی پایه از JPEG، PNG، WebP، AVIF، TIFF، BMP، GIF و PNM.
- مسیر توسعه برای JPEG XL و HEIF/HEIC با Backendهای Native اختیاری.
- مدیریت و بررسی Metadata و Privacy Mode.
- Hash رمزنگاری، dHash و aHash برای تشخیص فایل‌های یکسان و نزدیک.
- پایگاه‌داده SQLite با WAL.
- خروجی گزارش JSON، CSV و HTML.
- سیستم ساعت‌های مجاز پردازش، زمان فعلی، زمان باقی‌مانده تا پایان و زمان شروع پنجره بعدی.
- رابط Qt 6 با ظاهر نزدیک به Windows 11.
- تم‌های روشن، تاریک، پیش‌فرض ویندوز، آبی، قرمز و کنتراست بالا.
- زبان‌های انگلیسی، فارسی و چینی با مدیریت RTL/LTR.
- جداسازی کامل UI و Engine از طریق قرارداد JSON.

## ساختار پروژه
همان ساختار معرفی‌شده در بخش انگلیسی در فایل‌های `core/`، `gui/`، `scripts/`، `config/`، `docs/` و `tests/` قرار دارد.

## نصب و اجرا در Windows
```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\scripts\setup-windows.ps1
.\scripts\build.ps1
.\scripts\test.ps1
.\target\release\pixelforge.exe --help
```

## نصب و اجرا در Linux
```bash
chmod +x scripts/*.sh
./scripts/setup-linux.sh
./scripts/build.sh
./scripts/test.sh
./target/release/pixelforge --help
```

## دستورات نمونه
```text
pixelforge analyze image.jpg
pixelforge auto image.jpg
pixelforge compress image.jpg -o output.webp
pixelforge batch ./images -o ./optimized
pixelforge target image.jpg --size 500KB -o output.webp
pixelforge schedule estimate --config config/default.json --job-seconds 600
```

## ساعت کاری و زمان‌بندی
کاربر می‌تواند برای هر روز چند بازه زمانی وارد کند. سیستم تشخیص می‌دهد اکنون داخل بازه مجاز هست یا نه، چند ثانیه تا پایان آن باقی مانده، بازه بعدی چه زمانی شروع می‌شود و چند ثانیه تا آن فاصله وجود دارد. مدت تقریبی Job نیز به‌صورت ورودی قابل تعیین است.

نمونه:
```json
{
  "timezone":"local",
  "windows":[
    {"weekday":0,"start":"09:00","end":"17:00"},
    {"weekday":1,"start":"10:00","end":"18:00"},
    {"weekday":2,"start":"14:00","end":"22:30"}
  ],
  "queue_jobs_only_inside_windows":true
}
```

## رابط گرافیکی
Dashboard، Compression Workspace و Settings در هسته رابط قرار دارند. در Workspace فرمت خروجی، هدف فشرده‌سازی و بازه Quality تنظیم می‌شود. در Settings نیز تم، زبان، جهت متن و ساعت‌های مجاز پردازش قابل تنظیم است.

## معماری
رابط گرافیکی و CLI از طریق JSON با هسته Rust ارتباط دارند. مراحل اصلی شامل Validate، Decode، Analyze، Candidate Generation، Encode، Quality Evaluation، Verification و Save است.

## تست و CI
تست‌های Rust، پوشه Fixture برای تصاویر مرجع و Workflowهای CI برای Windows و Linux در پروژه قرار گرفته‌اند.

## مستندات
- `docs/ARCHITECTURE.md`
- `docs/DECISION_ENGINE.md`
- `docs/INSTALLATION_GUIDE.md`
- `docs/PLUGIN_SDK.md`
- `docs/SECURITY.md`
- `docs/FAQ.md`

## مجوز
MIT.

---

# 中文

## 项目简介
PIXELFORGE 是一个面向桌面与命令行自动化的专业图像压缩与优化平台。核心处理引擎使用 Rust，图形界面使用 C++/Qt 6。GUI、CLI 与未来自动化客户端共享同一处理流水线和 JSON 接口契约。

## 核心能力
- 压缩前深度图像分析。
- Fast Auto、Balanced Auto、Deep Auto。
- 无损、视觉无损、有损三种模式。
- 基于文件大小、压缩比例、质量与用户目标的多目标选择。
- Quality Guardian 输出验证。
- 单文件、批量、递归目录处理。
- Rayon 并行处理与单文件错误隔离。
- 目标文件大小与目标压缩比例。
- 基础支持 JPEG、PNG、WebP、AVIF、TIFF、BMP、GIF、PNM。
- 为 JPEG XL、HEIF/HEIC 提供可扩展的 Native Codec 后端接口。
- EXIF/隐私元数据处理基础。
- SHA-256、dHash、aHash。
- SQLite WAL 数据存储。
- JSON、CSV、HTML 报告。
- 可配置的处理时间窗口、当前时间判断、窗口结束倒计时、下一窗口开始时间与预计 Job 时长。
- Qt 6 原生桌面界面。
- Windows Default、Light、Dark、Blue、Red、High Contrast、AMOLED-style。
- English、فارسی、中文；英语/中文 LTR，波斯语 RTL，并提供方向覆盖。

## 安装：Windows
```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\scripts\setup-windows.ps1
.\scripts\build.ps1
.\scripts\test.ps1
.\target\release\pixelforge.exe --help
```

## 安装：Linux
```bash
chmod +x scripts/*.sh
./scripts/setup-linux.sh
./scripts/build.sh
./scripts/test.sh
./target/release/pixelforge --help
```

## CLI 示例
```text
pixelforge analyze image.jpg
pixelforge auto image.jpg
pixelforge compress image.jpg -o output.webp
pixelforge batch ./images -o ./optimized
pixelforge target image.jpg --size 500KB -o output.webp
pixelforge schedule estimate --config config/default.json --job-seconds 600
```

## 处理时间窗口
用户可以按星期设置多个开放时间区间。系统计算当前是否位于开放区间、当前区间剩余时间、下一个可用时间窗口以及距离下一窗口还有多少秒。预计 Job 时长可以作为参数输入，以便比较可用时间与任务 ETA。

JSON 示例：
```json
{
  "timezone":"local",
  "windows":[
    {"weekday":0,"start":"09:00","end":"17:00"},
    {"weekday":1,"start":"10:00","end":"18:00"},
    {"weekday":2,"start":"14:00","end":"22:30"}
  ],
  "queue_jobs_only_inside_windows":true
}
```

## GUI
GUI 采用 Qt 6，并提供 Dashboard、Compression Workspace、Batch Manager、Format Explorer、Presets、Reports、History 和 Settings 的界面扩展位。当前稳定主界面实现覆盖 Dashboard、Compression Workspace 与 Settings，后续页面继续通过同一 Qt/C++ 壳和 Rust JSON 引擎扩展。

## 架构
```text
Qt 6 GUI / CLI / Automation
            │
            ▼
     Versioned JSON API
            │
            ▼
      Rust Core Pipeline
            │
  Validate → Analyze → Candidates
            │
          Encode
            │
          Quality
            │
        Verify → Save
            │
          SQLite
```

## 文档
- `docs/ARCHITECTURE.md`
- `docs/DECISION_ENGINE.md`
- `docs/INSTALLATION_GUIDE.md`
- `docs/PLUGIN_SDK.md`
- `docs/SECURITY.md`
- `docs/FAQ.md`

## 测试
项目包含 Rust 测试、参考图像 Fixture 目录以及 Windows/Linux CI 工作流。

## 许可证
MIT。

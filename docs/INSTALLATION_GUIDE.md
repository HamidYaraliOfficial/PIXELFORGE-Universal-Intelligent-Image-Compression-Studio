# PIXELFORGE Installation Guide

## Windows 10/11
Install:
- Windows 10/11 x64
- Visual Studio 2022 Build Tools with Desktop C++ workload
- CMake 3.25+
- Ninja
- Rust stable via rustup
- Qt 6

PowerShell:
```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\scripts\setup-windows.ps1
.\scripts\build.ps1
.\scripts\test.ps1
```

Run CLI:
```powershell
.\target\release\pixelforge.exe --help
.\target\release\pixelforge.exe analyze .\photo.jpg
.\target\release\pixelforge.exe auto .\photo.jpg
.\target\release\pixelforge.exe compress .\photo.jpg -o .\out.webp
```

## Linux
Ubuntu/Debian:
```bash
chmod +x scripts/*.sh
./scripts/setup-linux.sh
./scripts/build.sh
./scripts/test.sh
```

Run:
```bash
./target/release/pixelforge analyze ./photo.png
./target/release/pixelforge batch ./images -o ./optimized
```

## Optional native codecs
Install native command-line codec suites when required by your workflow and point the plugin/backend layer at them using a deployment-specific environment variable. The baseline build remains self-contained for the formats covered by the Rust image backend.

## Verification
- `cargo test --workspace`
- `pixelforge analyze <file>`
- `pixelforge auto <file>`
- `pixelforge compress <file> -o <file>`

## Troubleshooting
If Qt cannot be found, set `CMAKE_PREFIX_PATH` to your Qt 6 installation.
If a DLL/shared library is missing, run the GUI from a shell that can see Qt's runtime directory or deploy it with Qt's deployment utility.
If an image is rejected, inspect the CLI error and confirm the container is not truncated and does not exceed safety limits.

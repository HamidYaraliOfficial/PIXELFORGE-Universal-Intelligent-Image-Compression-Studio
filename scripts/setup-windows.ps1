$ErrorActionPreference = "Stop"
Write-Host "PIXELFORGE Windows setup"
if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) { winget install --id Rustlang.Rustup -e }
if (-not (Get-Command cmake -ErrorAction SilentlyContinue)) { winget install --id Kitware.CMake -e }
if (-not (Get-Command ninja -ErrorAction SilentlyContinue)) { winget install --id Ninja-build.Ninja -e }
if (-not (Get-Command git -ErrorAction SilentlyContinue)) { winget install --id Git.Git -e }
Write-Host "Install Visual Studio 2022 Build Tools with C++ workload if not already present."
Write-Host "Install Qt 6 via official online installer or a managed Qt package source."
rustup toolchain install stable
cargo --version
cmake --version
ninja --version

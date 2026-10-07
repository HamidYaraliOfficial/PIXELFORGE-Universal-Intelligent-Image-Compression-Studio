#!/usr/bin/env bash
set -euo pipefail
echo "PIXELFORGE Linux setup"
if command -v apt-get >/dev/null 2>&1; then
  sudo apt-get update
  sudo apt-get install -y build-essential cmake ninja-build pkg-config libssl-dev git qt6-base-dev
fi
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi
rustup toolchain install stable
rustc --version
cmake --version
ninja --version
qmake6 --version || true

#!/usr/bin/env bash
set -euo pipefail
cargo build --workspace --release
cmake -S gui -B build/gui -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build/gui --config Release
echo "PIXELFORGE build completed."

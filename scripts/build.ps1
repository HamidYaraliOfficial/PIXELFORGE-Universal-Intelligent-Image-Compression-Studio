$ErrorActionPreference = "Stop"
cargo build --workspace --release
cmake -S gui -B build/gui -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build/gui --config Release
Write-Host "PIXELFORGE build completed."

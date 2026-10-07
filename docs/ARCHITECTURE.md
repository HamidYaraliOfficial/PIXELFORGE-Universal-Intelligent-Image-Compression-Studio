# PIXELFORGE Architecture

## Layers
- `core/`: Rust shared engine and CLI.
- `analysis/`: bounded image sampling, entropy, texture, edge, color, alpha and perceptual hashes.
- `codecs/`: codec adapter boundary; the current baseline uses the Rust `image` backend, with explicit extension points for native codec plugins.
- `compression/`: concrete encoder calls and temporary-file handling.
- `optimizer/`: content-aware candidate generation and objective scoring.
- `quality/`: PSNR/SSIM/MS-SSIM style verification.
- `metadata/`: EXIF inspection and privacy-oriented policy layer.
- `pipeline/`: Decode → Normalize → Analyze → Generate → Encode → Evaluate → Verify → Save.
- `database/`: SQLite schema and WAL.
- `rules/`: deterministic batch rule expressions.
- `scheduler/`: operating windows, next-window calculation and ETA.
- `report/`: JSON/CSV/HTML exports.
- `gui/`: Qt 6 native desktop shell. It launches the versioned Rust CLI contract without putting CPU-heavy work on the UI thread.

## Processing contract
The engine communicates using JSON records on stdout. GUI and automation can therefore share the same executable and configuration model.

## Resource controls
The pipeline rejects inputs beyond a safety ceiling and samples large images before complete materialization. Batch jobs are parallelized through Rayon and failures are isolated at the file level.

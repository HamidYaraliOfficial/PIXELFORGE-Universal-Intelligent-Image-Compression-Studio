# Security Model

PIXELFORGE treats every image as untrusted input.

Controls:
- size ceiling before decode;
- path normalization and no implicit recursive writes outside the requested destination;
- temporary files with random identifiers;
- final output written through a `.part` file and renamed atomically;
- integer-safe arithmetic for target and ratio calculations;
- per-file batch isolation;
- metadata privacy mode;
- no execution of image file contents;
- JSON-only engine protocol;
- structured errors and no queue-wide failure on one corrupted image.

For server/headless deployments, run with OS-level process limits, filesystem sandboxing and a low-privilege service account.

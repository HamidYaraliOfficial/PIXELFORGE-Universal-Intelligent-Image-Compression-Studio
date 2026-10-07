# Compression Decision Engine

1. Validate the container and file-size limits.
2. Decode only after basic validation.
3. Extract geometry, alpha, bit depth, histogram, entropy, gradients, flat-region ratio, texture, noise, sharpness, contrast, dynamic range, line density and content scores.
4. Generate a small candidate set based on Fast/Balanced/Deep Auto.
5. Encode every candidate into a temporary file.
6. Decode each candidate and compute PSNR, SSIM, MS-SSIM style score and absolute-error distance.
7. Reject candidates below the quality guardian threshold.
8. Score remaining candidates against the user's goal.
9. Save the selected candidate atomically.
10. Re-decode the final artifact for verification.

The engine uses deterministic feature extraction and deterministic candidate generation so the same input/configuration produces repeatable choices on the same build.

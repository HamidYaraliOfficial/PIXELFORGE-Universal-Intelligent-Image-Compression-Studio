use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

use crate::QualityMetrics;

pub fn evaluate(original: &DynamicImage, candidate_path: &Path) -> Result<QualityMetrics> {
    let cand = image::ImageReader::open(candidate_path)?.with_guessed_format()?.decode()?;
    let (ow, oh) = original.dimensions();
    let cand = if cand.dimensions() != (ow, oh) { cand.resize_exact(ow, oh, image::imageops::FilterType::Lanczos3) } else { cand };
    let o = original.to_rgb8();
    let c = cand.to_rgb8();
    let n = (ow as usize) * (oh as usize);
    let mut mse = 0.0f64;
    let mut mae = 0.0f64;
    let mut ssim_sum = 0.0f64;
    for y in 0..oh {
        for x in 0..ow {
            let a = o.get_pixel(x, y).0;
            let b = c.get_pixel(x, y).0;
            for k in 0..3 {
                let d = a[k] as f64 - b[k] as f64;
                mse += d * d;
                mae += d.abs();
            }
            let ax = (a[0] as f64 + a[1] as f64 + a[2] as f64) / 3.0;
            let bx = (b[0] as f64 + b[1] as f64 + b[2] as f64) / 3.0;
            let c1 = 6.5025;
            ssim_sum += ((2.0 * ax * bx + c1) / (ax*ax + bx*bx + c1)).clamp(0.0, 1.0);
        }
    }
    mse /= (n.max(1) * 3) as f64;
    mae /= (n.max(1) * 3) as f64;
    let psnr = if mse == 0.0 { 99.0 } else { 10.0 * ((255.0*255.0)/mse).log10() };
    let ssim = (ssim_sum / n.max(1) as f64) as f32;
    let ms_ssim = (ssim * 0.55 + (1.0 - mae as f32 / 255.0) * 0.45).clamp(0.0, 1.0);
    Ok(QualityMetrics {
        psnr: psnr as f32,
        ssim,
        ms_ssim,
        perceptual_distance: (mae / 255.0) as f32,
    })
}

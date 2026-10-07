use anyhow::Result;
use image::{DynamicImage, GenericImageView, Pixel};
use sha2::{Digest, Sha256};
use std::path::Path;
use crate::{ImageAnalysis, util};

pub fn analyze_path(path: &Path) -> Result<ImageAnalysis> {
    let bytes = std::fs::read(path)?;
    let crypto_hash = format!("{:x}", Sha256::digest(&bytes));
    let file_size = bytes.len() as u64;
    let img = image::ImageReader::open(path)?.with_guessed_format()?.decode()?;
    analyze_image(&img, file_size, util::mime_for_path(path), util::format_for_path(path), crypto_hash)
}

pub fn analyze_image(
    img: &DynamicImage,
    file_size: u64,
    mime_type: String,
    format: String,
    crypto_hash: String,
) -> Result<ImageAnalysis> {
    let (w, h) = img.dimensions();
    let rgba = img.to_rgba8();
    let mut luma = Vec::with_capacity((w.min(2048) * h.min(2048)) as usize);
    let step_x = (w as f32 / 2048.0).ceil() as u32;
    let step_y = (h as f32 / 2048.0).ceil() as u32;
    let mut alpha_transparent = 0u64;
    let mut hist = [0u64; 256];
    let mut sum = 0.0f64;
    let mut sum2 = 0.0f64;
    let mut edges = 0u64;
    let mut gradients = 0u64;
    let mut flat = 0u64;
    let mut samples = 0u64;
    let mut text_like = 0u64;
    let mut line_like = 0u64;

    for y in (0..h).step_by(step_y.max(1) as usize) {
        let mut prev = 0.0f32;
        for x in (0..w).step_by(step_x.max(1) as usize) {
            let p = rgba.get_pixel(x, y).channels();
            if p[3] < 250 { alpha_transparent += 1; }
            let yy = 0.2126 * p[0] as f32 + 0.7152 * p[1] as f32 + 0.0722 * p[2] as f32;
            luma.push(yy);
            let idx = yy.clamp(0.0, 255.0) as usize;
            hist[idx] += 1;
            sum += yy as f64;
            sum2 += (yy as f64).powi(2);
            if samples > 0 {
                let d = (yy - prev).abs();
                if d > 24.0 { edges += 1; }
                if d > 10.0 && d < 80.0 { gradients += 1; }
                if d > 50.0 { text_like += 1; }
            }
            prev = yy;
            samples += 1;
        }
    }

    for i in 1..hist.len() {
        let a = hist[i - 1] as f64;
        let b = hist[i] as f64;
        if (a - b).abs() > (samples as f64 * 0.01) { line_like += 1; }
    }
    let mean = (sum / samples.max(1) as f64) as f32;
    let var = (sum2 / samples.max(1) as f64 - (mean as f64).powi(2)).max(0.0) as f32;
    let stddev = var.sqrt();
    let entropy = hist.iter().filter(|v| **v > 0).map(|v| {
        let p = *v as f32 / samples.max(1) as f32;
        -p * p.log2()
    }).sum::<f32>();
    let contrast = (stddev / 64.0).clamp(0.0, 1.0);
    let dynamic_range = dynamic_range(&hist);
    let edge_density = (edges as f32 / samples.max(1) as f32).clamp(0.0, 1.0);
    let gradient_complexity = (gradients as f32 / samples.max(1) as f32).clamp(0.0, 1.0);
    let text_density = (text_like as f32 / samples.max(1) as f32).clamp(0.0, 1.0);
    let line_density = (line_like as f32 / 255.0).clamp(0.0, 1.0);
    let flat_region_ratio = (1.0 - edge_density - gradient_complexity * 0.3).clamp(0.0, 1.0);
    let texture_complexity = (entropy / 8.0 * 0.6 + edge_density * 0.4).clamp(0.0, 1.0);
    let noise_level = estimate_noise(&luma).clamp(0.0, 1.0);
    let sharpness = estimate_sharpness(&luma).clamp(0.0, 1.0);
    let colors = approximate_colors(&rgba);
    let detail_score = (texture_complexity * 0.5 + sharpness * 0.3 + contrast * 0.2).clamp(0.0, 1.0);
    let photo_score = (0.45 + texture_complexity * 0.35 + contrast * 0.2 - text_density * 0.3).clamp(0.0, 1.0);
    let illustration_score = (flat_region_ratio * 0.55 + (1.0 - noise_level) * 0.2 + (1.0 - colors as f32 / 65536.0).clamp(0.0, 1.0) * 0.25).clamp(0.0, 1.0);
    let screenshot_score = (text_density * 0.55 + line_density * 0.25 + flat_region_ratio * 0.2).clamp(0.0, 1.0);
    let color_space = if img.color() == image::ColorType::L8 { "GRAY" } else { "sRGB" }.to_string();
    let has_alpha = img.color().has_alpha();
    let transparency_ratio = alpha_transparent as f32 / samples.max(1) as f32;

    Ok(ImageAnalysis {
        width: w,
        height: h,
        aspect_ratio: w as f32 / h.max(1) as f32,
        file_size,
        mime_type,
        format,
        color_space,
        channels: img.color().channel_count(),
        bit_depth: img.color().bits_per_pixel().min(16) as u8,
        has_alpha,
        approximate_colors: colors,
        entropy,
        edge_density,
        texture_complexity,
        noise_level,
        sharpness,
        contrast,
        dynamic_range,
        gradient_complexity,
        flat_region_ratio,
        photo_score,
        illustration_score,
        screenshot_score,
        text_density,
        line_density,
        detail_score,
        transparency_ratio,
        d_hash: util::d_hash(&luma),
        a_hash: util::a_hash(&luma),
        crypto_hash,
    })
}

fn dynamic_range(hist: &[u64; 256]) -> f32 {
    let total: u64 = hist.iter().sum();
    let mut lo = 0usize;
    let mut hi = 255usize;
    while lo < 255 && hist[lo] == 0 { lo += 1; }
    while hi > 0 && hist[hi] == 0 { hi -= 1; }
    if total == 0 { 0.0 } else { (hi.saturating_sub(lo) as f32 / 255.0).clamp(0.0, 1.0) }
}

fn approximate_colors(img: &image::RgbaImage) -> u32 {
    let mut set = std::collections::HashSet::with_capacity(8192);
    let sx = (img.width() / 128).max(1);
    let sy = (img.height() / 128).max(1);
    for y in (0..img.height()).step_by(sy as usize) {
        for x in (0..img.width()).step_by(sx as usize) {
            let p = img.get_pixel(x, y).0;
            set.insert(((p[0] as u32 >> 4) << 8) | ((p[1] as u32 >> 4) << 4) | (p[2] as u32 >> 4));
            if set.len() > 200_000 { return 200_000; }
        }
    }
    set.len() as u32
}

fn estimate_noise(v: &[f32]) -> f32 {
    if v.len() < 3 { return 0.0; }
    let mut d = Vec::with_capacity(v.len() - 2);
    for i in 1..v.len()-1 {
        d.push((v[i] - (v[i-1] + v[i+1]) * 0.5).abs());
    }
    (d.iter().sum::<f32>() / d.len().max(1) as f32 / 32.0).clamp(0.0, 1.0)
}

fn estimate_sharpness(v: &[f32]) -> f32 {
    if v.len() < 2 { return 0.0; }
    let mut e = 0.0f32;
    for i in 1..v.len() { e += (v[i] - v[i-1]).abs(); }
    (e / (v.len() as f32) / 64.0).clamp(0.0, 1.0)
}

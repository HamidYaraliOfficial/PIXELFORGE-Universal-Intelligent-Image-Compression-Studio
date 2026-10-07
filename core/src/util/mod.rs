use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use std::path::Path;

pub fn mime_for_path(path: &Path) -> String {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "jxl" => "image/jxl",
        "tif" | "tiff" => "image/tiff",
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        "heic" | "heif" => "image/heif",
        "ppm" => "image/x-portable-pixmap",
        "pgm" => "image/x-portable-graymap",
        "pbm" => "image/x-portable-bitmap",
        "pnm" => "image/x-portable-anymap",
        _ => "application/octet-stream",
    }.to_string()
}

pub fn format_for_path(path: &Path) -> String {
    path.extension().and_then(|s| s.to_str()).unwrap_or("unknown").to_uppercase()
}

pub fn output_extension(fmt: &crate::OutputFormat, input: &Path) -> String {
    match fmt {
        crate::OutputFormat::Jpeg => "jpg",
        crate::OutputFormat::Png => "png",
        crate::OutputFormat::Webp => "webp",
        crate::OutputFormat::Avif => "avif",
        crate::OutputFormat::Jxl => "jxl",
        crate::OutputFormat::Tiff => "tiff",
        crate::OutputFormat::Bmp => "bmp",
        crate::OutputFormat::Gif => "gif",
        crate::OutputFormat::Heif => "heic",
        crate::OutputFormat::Pnm => "pnm",
        crate::OutputFormat::Keep | crate::OutputFormat::Auto => input.extension().and_then(|x| x.to_str()).unwrap_or("out"),
    }.into()
}

pub fn load(path: &Path) -> Result<DynamicImage> {
    Ok(image::ImageReader::open(path)?.with_guessed_format()?.decode()?)
}

pub fn d_hash(samples: &[f32]) -> String {
    if samples.len() < 2 { return "0".repeat(16); }
    let mut out = 0u64;
    for i in 0..64 {
        let a = samples[(i * 66) % samples.len()];
        let b = samples[(i * 66 + 1) % samples.len()];
        out <<= 1;
        if a > b { out |= 1; }
    }
    format!("{out:016x}")
}

pub fn a_hash(samples: &[f32]) -> String {
    if samples.is_empty() { return "0".repeat(16); }
    let avg = samples.iter().sum::<f32>() / samples.len() as f32;
    let mut out = 0u64;
    for i in 0..64 {
        let v = samples[(i * 67) % samples.len()];
        out <<= 1;
        if v >= avg { out |= 1; }
    }
    format!("{out:016x}")
}

pub fn image_format_for_output(fmt: &crate::OutputFormat) -> Option<ImageFormat> {
    Some(match fmt {
        crate::OutputFormat::Jpeg => ImageFormat::Jpeg,
        crate::OutputFormat::Png => ImageFormat::Png,
        crate::OutputFormat::Webp => ImageFormat::WebP,
        crate::OutputFormat::Tiff => ImageFormat::Tiff,
        crate::OutputFormat::Bmp => ImageFormat::Bmp,
        crate::OutputFormat::Gif => ImageFormat::Gif,
        crate::OutputFormat::Pnm => ImageFormat::Pnm,
        _ => return None,
    })
}

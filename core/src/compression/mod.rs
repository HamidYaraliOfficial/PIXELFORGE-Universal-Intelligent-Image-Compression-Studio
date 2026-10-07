use anyhow::{anyhow, Result};
use image::{codecs::jpeg::JpegEncoder, ColorType, DynamicImage, ImageEncoder};
use std::fs::File;
use std::path::Path;
use crate::{CandidateConfig, OutputFormat};
use crate::util;

pub fn encode(img: &DynamicImage, candidate: &CandidateConfig, output: &Path) -> Result<()> {
    match candidate.format {
        OutputFormat::Jpeg => encode_jpeg(img, candidate.quality, output),
        OutputFormat::Png => encode_png(img, candidate.quality, output),
        OutputFormat::Webp => encode_webp(img, candidate.quality, output),
        OutputFormat::Tiff => encode_with_image(img, image::ImageFormat::Tiff, output),
        OutputFormat::Bmp => encode_with_image(img, image::ImageFormat::Bmp, output),
        OutputFormat::Gif => encode_with_image(img, image::ImageFormat::Gif, output),
        OutputFormat::Pnm => encode_with_image(img, image::ImageFormat::Pnm, output),
        OutputFormat::Avif => encode_with_image(img, image::ImageFormat::Avif, output),
        OutputFormat::Jxl | OutputFormat::Heif => {
            Err(anyhow!("JXL/HEIF require optional system codec backend; configure PIXELFORGE_CODEC_PATH or install the matching CLI backend"))
        }
        OutputFormat::Auto | OutputFormat::Keep => {
            let f = util::image_format_for_output(&candidate.format);
            if let Some(fmt) = f { encode_with_image(img, fmt, output) } else { Err(anyhow!("unsupported automatic output format")) }
        }
    }
}

fn encode_jpeg(img: &DynamicImage, quality: u8, output: &Path) -> Result<()> {
    let mut file = File::create(output)?;
    let rgb = img.to_rgb8();
    let enc = JpegEncoder::new_with_quality(&mut file, quality);
    enc.write_image(rgb.as_raw(), rgb.width(), rgb.height(), ColorType::Rgb8.into())?;
    Ok(())
}

fn encode_png(img: &DynamicImage, _quality: u8, output: &Path) -> Result<()> {
    let file = File::create(output)?;
    let mut enc = image::codecs::png::PngEncoder::new_with_quality(
        file,
        image::codecs::png::CompressionType::Best,
        image::codecs::png::FilterType::Adaptive
    );
    let rgba = img.to_rgba8();
    enc.write_image(rgba.as_raw(), rgba.width(), rgba.height(), ColorType::Rgba8.into())?;
    Ok(())
}

fn encode_webp(img: &DynamicImage, quality: u8, output: &Path) -> Result<()> {
    let rgb = img.to_rgb8();
    let file = File::create(output)?;
    let mut enc = image::codecs::webp::WebPEncoder::new_lossy(file, quality as f32);
    enc.write_image(rgb.as_raw(), rgb.width(), rgb.height(), ColorType::Rgb8.into())?;
    Ok(())
}

fn encode_with_image(img: &DynamicImage, fmt: image::ImageFormat, output: &Path) -> Result<()> {
    img.save_with_format(output, fmt)?;
    Ok(())
}

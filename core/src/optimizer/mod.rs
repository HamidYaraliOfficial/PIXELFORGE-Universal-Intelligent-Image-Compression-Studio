use crate::{AutoDepth, CandidateConfig, CompressionConfig, CompressionGoal, ImageAnalysis, OutputFormat, QualityMode};

pub fn choose_format(a: &ImageAnalysis, c: &CompressionConfig) -> OutputFormat {
    if !matches!(c.output_format, OutputFormat::Auto) {
        return c.output_format.clone();
    }
    if matches!(c.quality_mode, QualityMode::Lossless) {
        if a.has_alpha || a.illustration_score > a.photo_score { return OutputFormat::Png; }
        return if a.format == "TIFF" { OutputFormat::Tiff } else { OutputFormat::Png };
    }
    if a.has_alpha || a.transparency_ratio > 0.01 {
        if a.screenshot_score > 0.65 || a.illustration_score > a.photo_score {
            return OutputFormat::Webp;
        }
        return OutputFormat::Avif;
    }
    if a.screenshot_score > 0.70 && a.text_density > 0.25 {
        return OutputFormat::Webp;
    }
    if a.photo_score > 0.55 && a.dynamic_range > 0.35 {
        return match c.goal {
            CompressionGoal::MaximumQuality => OutputFormat::Jpeg,
            _ => OutputFormat::Webp,
        };
    }
    if a.illustration_score > 0.60 { return OutputFormat::Webp; }
    OutputFormat::Webp
}

pub fn generate_candidates(a: &ImageAnalysis, c: &CompressionConfig) -> Vec<CandidateConfig> {
    let format = choose_format(a, c);
    let mut list = Vec::new();
    let (lo, hi, count) = match c.auto_depth {
        AutoDepth::FastAuto => (c.quality_min, c.quality_max, 3),
        AutoDepth::BalancedAuto => (c.quality_min, c.quality_max, 6),
        AutoDepth::DeepAuto => (c.quality_min, c.quality_max, 10),
    };
    let step = ((hi.saturating_sub(lo) as usize / count.max(1)) as u8).max(1);
    let mut q = lo;
    while q <= hi && list.len() < count {
        list.push(CandidateConfig {
            format: format.clone(),
            quality: q,
            effort: c.effort,
            chroma_subsampling: if a.screenshot_score > 0.7 { "4:4:4".into() } else { c.chroma_subsampling.clone() },
            progressive: c.progressive,
            reason: reason_for(a, &format, q),
        });
        if hi - q < step { break; }
        q += step;
    }
    list
}

fn reason_for(a: &ImageAnalysis, format: &OutputFormat, q: u8) -> String {
    let type_note = if a.photo_score >= a.screenshot_score && a.photo_score >= a.illustration_score {
        "photo/texture characteristics"
    } else if a.screenshot_score >= a.illustration_score {
        "text/line/screenshot characteristics"
    } else {
        "flat-color/illustration characteristics"
    };
    format!("{} with {} quality and {}px-class input", format_name(format), q, (a.width.max(a.height) / 1000) * 1000) + &format!("; selected from {type_note}")
}

fn format_name(v: &OutputFormat) -> &'static str {
    match v {
        OutputFormat::Jpeg => "JPEG", OutputFormat::Png => "PNG", OutputFormat::Webp => "WebP",
        OutputFormat::Avif => "AVIF", OutputFormat::Jxl => "JPEG XL", OutputFormat::Tiff => "TIFF",
        OutputFormat::Bmp => "BMP", OutputFormat::Gif => "GIF", OutputFormat::Heif => "HEIF",
        OutputFormat::Pnm => "PNM", OutputFormat::Keep => "original format", OutputFormat::Auto => "automatic format",
    }
}

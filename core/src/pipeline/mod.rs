use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::{path::{Path, PathBuf}, time::Instant};
use uuid::Uuid;

use crate::{
    analysis, compression, optimizer, quality, BatchRequest, CompressionConfig, CompressionRun,
    CandidateConfig, OutputFormat
};
use crate::util;

pub fn run_one(input: &Path, output: &Path, config: &CompressionConfig) -> Result<CompressionRun> {
    validate_input(input)?;
    let start = Instant::now();
    let original = util::load(input)?;
    let analysis = analysis::analyze_path(input)?;
    let candidates = optimizer::generate_candidates(&analysis, config);
    if candidates.is_empty() { return Err(anyhow!("no candidate configurations generated")); }

    let temp_dir = output.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(temp_dir)?;
    let mut best: Option<(CandidateConfig, quality::QualityMetrics, u64)> = None;

    for (idx, cand) in candidates.iter().enumerate() {
        let tmp = temp_dir.join(format!(".pixelforge-{}-{}.tmp", Uuid::new_v4(), idx));
        compression::encode(&original, cand, &tmp)?;
        let size = std::fs::metadata(&tmp)?.len();
        let metrics = quality::evaluate(&original, &tmp)?;
        let acceptable = metrics.ssim >= quality_threshold(config) && metrics.perceptual_distance <= config.max_quality_loss.max(0.1) / 100.0;
        let score = objective_score(config, analysis.file_size, size, &metrics, acceptable);
        let choose = match &best {
            None => true,
            Some((_, oldm, olds)) => score > objective_score(config, analysis.file_size, *olds, oldm, true),
        };
        if choose { best = Some((cand.clone(), metrics, size)); }
        std::fs::remove_file(&tmp).ok();
    }

    let (selected, _metrics, _size) = best.ok_or_else(|| anyhow!("no candidate was usable"))?;
    let temp_final = output.with_extension("pixelforge.part");
    compression::encode(&original, &selected, &temp_final)?;
    let verified = quality::evaluate(&original, &temp_final).is_ok();
    if !verified {
        std::fs::remove_file(&temp_final).ok();
        return Err(anyhow!("quality verification could not decode the result"));
    }
    std::fs::rename(&temp_final, output)?;
    let final_size = std::fs::metadata(output)?.len();
    let metrics = quality::evaluate(&original, output)?;
    let saved = analysis.file_size as i64 - final_size as i64;
    let reduction = if analysis.file_size == 0 { 0.0 } else { saved as f32 * 100.0 / analysis.file_size as f32 };

    Ok(CompressionRun {
        id: Uuid::new_v4(),
        input: input.display().to_string(),
        output: output.display().to_string(),
        original_size: analysis.file_size,
        final_size,
        saved_bytes: saved,
        reduction_percent: reduction,
        selected: selected.clone(),
        metrics,
        encoding_time_ms: start.elapsed().as_millis(),
        decoder_verified: verified,
        decision_reason: selected.reason.clone(),
    })
}

fn quality_threshold(config: &CompressionConfig) -> f32 {
    match config.quality_mode {
        crate::QualityMode::Lossless => 0.999,
        crate::QualityMode::VisuallyLossless => 0.985,
        crate::QualityMode::Lossy => 0.94,
    }
}

fn objective_score(
    config: &CompressionConfig,
    original: u64,
    final_size: u64,
    metrics: &crate::QualityMetrics,
    acceptable: bool,
) -> f32 {
    let reduction = if original == 0 { 0.0 } else { 1.0 - final_size as f32 / original as f32 };
    let quality = (metrics.ssim * 0.55 + metrics.ms_ssim * 0.35 + (metrics.psnr / 60.0).min(1.0) * 0.10).clamp(0.0, 1.0);
    let penalty = if acceptable { 0.0 } else { 0.35 };
    match config.goal {
        crate::CompressionGoal::MaximumCompression => reduction * 0.78 + quality * 0.22 - penalty,
        crate::CompressionGoal::MaximumQuality => quality * 0.85 + reduction * 0.15 - penalty,
        crate::CompressionGoal::WebOptimized => reduction * 0.55 + quality * 0.35 + (final_size < 500_000) as i32 as f32 * 0.10 - penalty,
        crate::CompressionGoal::TargetFileSize { bytes } => {
            let distance = (final_size as f64 - bytes as f64).abs() / bytes.max(1) as f64;
            (1.0 - distance as f32).clamp(0.0, 1.0) * 0.65 + quality * 0.35 - penalty
        },
        crate::CompressionGoal::TargetReduction { percent } => {
            let distance = (reduction * 100.0 - percent).abs();
            (1.0 - (distance / 100.0)).clamp(0.0, 1.0) * 0.65 + quality * 0.35 - penalty
        },
        crate::CompressionGoal::Balanced => reduction * 0.50 + quality * 0.50 - penalty,
    }
}

pub fn run_batch(request: &BatchRequest) -> Result<Vec<CompressionRun>> {
    let inputs = collect_inputs(&request.input, request.recursive)?;
    std::fs::create_dir_all(&request.output_dir)?;
    let results: Vec<_> = inputs.par_iter().map(|p| {
        let name = p.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
        let ext = util::output_extension(&request.config.output_format, p);
        let out = request.output_dir.join(format!("{}.{}", name, ext));
        run_one(p, &out, &request.config)
    }).collect();
    let mut ok = Vec::new();
    for r in results {
        if let Ok(v) = r { ok.push(v); }
    }
    Ok(ok)
}

fn collect_inputs(path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    if path.is_file() { return Ok(vec![path.to_path_buf()]); }
    let mut out = Vec::new();
    let mut it = walkdir::WalkDir::new(path).follow_links(false);
    if !recursive { it = it.max_depth(1); }
    for e in it {
        let e = e?;
        if e.file_type().is_file() && is_supported(e.path()) { out.push(e.path().to_path_buf()); }
    }
    Ok(out)
}

fn is_supported(p: &Path) -> bool {
    matches!(p.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase().as_str(),
        "jpg"|"jpeg"|"png"|"webp"|"avif"|"tif"|"tiff"|"bmp"|"gif"|"ppm"|"pgm"|"pbm"|"pnm")
}
fn validate_input(p: &Path) -> Result<()> {
    if !p.exists() { return Err(anyhow!("input does not exist")); }
    let meta = std::fs::metadata(p)?;
    if meta.len() > 25 * 1024 * 1024 * 1024u64 { return Err(anyhow!("input exceeds safety size limit")); }
    Ok(())
}

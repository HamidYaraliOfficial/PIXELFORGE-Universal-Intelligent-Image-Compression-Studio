use anyhow::Result;
use crate::CompressionRun;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct BatchReport {
    pub run_count: usize,
    pub original_size: u64,
    pub final_size: u64,
    pub saved_bytes: i64,
    pub average_reduction: f32,
    pub average_ssim: f32,
    pub total_processing_time_ms: u128,
    pub errors: Vec<String>,
}

pub fn batch(runs: &[CompressionRun], errors: Vec<String>) -> BatchReport {
    let original = runs.iter().map(|r| r.original_size).sum();
    let final_size = runs.iter().map(|r| r.final_size).sum();
    let saved = original as i64 - final_size as i64;
    let avg_red = if runs.is_empty() { 0.0 } else { runs.iter().map(|r| r.reduction_percent).sum::<f32>() / runs.len() as f32 };
    let avg_ssim = if runs.is_empty() { 0.0 } else { runs.iter().map(|r| r.metrics.ssim).sum::<f32>() / runs.len() as f32 };
    BatchReport { run_count: runs.len(), original_size: original, final_size, saved_bytes: saved, average_reduction: avg_red, average_ssim: avg_ssim, total_processing_time_ms: runs.iter().map(|r| r.encoding_time_ms).sum(), errors }
}

pub fn write_json<T: Serialize>(value: &T, path: &Path) -> Result<()> {
    std::fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

pub fn write_csv(runs: &[CompressionRun], path: &Path) -> Result<()> {
    let mut s = String::from("input,output,original_size,final_size,saved_bytes,reduction_percent,format,quality,ssim,psnr,elapsed_ms\n");
    for r in runs {
        s.push_str(&format!(
            "\"{}\",\"{}\",{},{},{},{:.3},\"{:?}\",{},{:.5},{:.3},{}\n",
            r.input.replace('"', "\"\""), r.output.replace('"', "\"\""),
            r.original_size, r.final_size, r.saved_bytes, r.reduction_percent,
            r.selected.format, r.selected.quality, r.metrics.ssim, r.metrics.psnr, r.encoding_time_ms
        ));
    }
    std::fs::write(path, s)?;
    Ok(())
}

pub fn write_html(runs: &[CompressionRun], path: &Path) -> Result<()> {
    let mut rows = String::new();
    for r in runs {
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.2}%</td><td>{:.4}</td><td>{:?}</td><td>{}</td></tr>",
            html_escape(&r.input), html_escape(&r.output), r.final_size, r.reduction_percent,
            r.metrics.ssim, r.selected.format, r.selected.quality
        ));
    }
    let html = format!(r#"<!doctype html><html lang="en"><meta charset="utf-8"><title>PIXELFORGE Report</title>
    <style>body{{font-family:system-ui;margin:2rem}}table{{border-collapse:collapse;width:100%}}td,th{{padding:.6rem;border:1px solid #ccc}}</style>
    <h1>PIXELFORGE Compression Report</h1><table><thead><tr><th>Input</th><th>Output</th><th>Final Bytes</th><th>Reduction</th><th>SSIM</th><th>Format</th><th>Quality</th></tr></thead><tbody>{rows}</tbody></table></html>"#);
    std::fs::write(path, html)?;
    Ok(())
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

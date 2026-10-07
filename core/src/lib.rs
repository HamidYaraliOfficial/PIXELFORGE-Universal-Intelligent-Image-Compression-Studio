pub mod analysis;
pub mod compression;
pub mod database;
pub mod metadata;
pub mod optimizer;
pub mod pipeline;
pub mod quality;
pub mod report;
pub mod rules;
pub mod scheduler;
pub mod util;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionGoal {
    MaximumCompression,
    Balanced,
    MaximumQuality,
    TargetFileSize { bytes: u64 },
    TargetReduction { percent: f32 },
    WebOptimized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityMode {
    Lossless,
    VisuallyLossless,
    Lossy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Auto,
    Jpeg,
    Png,
    Webp,
    Avif,
    Jxl,
    Tiff,
    Bmp,
    Gif,
    Heif,
    Pnm,
    Keep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetadataPolicy {
    KeepAll,
    KeepSelected { fields: Vec<String> },
    RemoveAll,
    Privacy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutoDepth {
    FastAuto,
    BalancedAuto,
    DeepAuto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub goal: CompressionGoal,
    pub quality_mode: QualityMode,
    pub output_format: OutputFormat,
    pub auto_depth: AutoDepth,
    pub quality_min: u8,
    pub quality_max: u8,
    pub max_quality_loss: f32,
    pub target_width: Option<u32>,
    pub target_height: Option<u32>,
    pub max_megapixels: Option<f64>,
    pub dpi: Option<f32>,
    pub progressive: bool,
    pub optimize_huffman: bool,
    pub chroma_subsampling: String,
    pub effort: u8,
    pub metadata_policy: MetadataPolicy,
    pub remove_gps: bool,
    pub privacy_mode: bool,
    pub parallelism: Option<usize>,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            goal: CompressionGoal::Balanced,
            quality_mode: QualityMode::VisuallyLossless,
            output_format: OutputFormat::Auto,
            auto_depth: AutoDepth::BalancedAuto,
            quality_min: 60,
            quality_max: 92,
            max_quality_loss: 2.0,
            target_width: None,
            target_height: None,
            max_megapixels: None,
            dpi: None,
            progressive: true,
            optimize_huffman: true,
            chroma_subsampling: "4:2:0".into(),
            effort: 5,
            metadata_policy: MetadataPolicy::Privacy,
            remove_gps: true,
            privacy_mode: true,
            parallelism: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub config: CompressionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub file_size: u64,
    pub mime_type: String,
    pub format: String,
    pub color_space: String,
    pub channels: u8,
    pub bit_depth: u8,
    pub has_alpha: bool,
    pub approximate_colors: u32,
    pub entropy: f32,
    pub edge_density: f32,
    pub texture_complexity: f32,
    pub noise_level: f32,
    pub sharpness: f32,
    pub contrast: f32,
    pub dynamic_range: f32,
    pub gradient_complexity: f32,
    pub flat_region_ratio: f32,
    pub photo_score: f32,
    pub illustration_score: f32,
    pub screenshot_score: f32,
    pub text_density: f32,
    pub line_density: f32,
    pub detail_score: f32,
    pub transparency_ratio: f32,
    pub d_hash: String,
    pub a_hash: String,
    pub crypto_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateConfig {
    pub format: OutputFormat,
    pub quality: u8,
    pub effort: u8,
    pub chroma_subsampling: String,
    pub progressive: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub psnr: f32,
    pub ssim: f32,
    pub ms_ssim: f32,
    pub perceptual_distance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionRun {
    pub id: Uuid,
    pub input: String,
    pub output: String,
    pub original_size: u64,
    pub final_size: u64,
    pub saved_bytes: i64,
    pub reduction_percent: f32,
    pub selected: CandidateConfig,
    pub metrics: QualityMetrics,
    pub encoding_time_ms: u128,
    pub decoder_verified: bool,
    pub decision_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleWindow {
    pub weekday: u8,
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchedulerConfig {
    pub timezone: String,
    pub windows: Vec<ScheduleWindow>,
    pub queue_jobs_only_inside_windows: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRule {
    pub name: String,
    pub expression: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequest {
    pub input: PathBuf,
    pub output_dir: PathBuf,
    pub config: CompressionConfig,
    pub recursive: bool,
    pub rules: Vec<BatchRule>,
    pub schedule: Option<SchedulerConfig>,
}

pub fn compress_file(input: &Path, output: &Path, config: &CompressionConfig) -> Result<CompressionRun> {
    pipeline::run_one(input, output, config)
        .with_context(|| format!("compression failed for {}", input.display()))
}

pub fn analyze_file(input: &Path) -> Result<ImageAnalysis> {
    analysis::analyze_path(input)
}

pub fn auto_candidates(input: &Path, config: &CompressionConfig) -> Result<Vec<CandidateConfig>> {
    let a = analyze_file(input)?;
    Ok(optimizer::generate_candidates(&a, config))
}

pub fn batch_compress(request: &BatchRequest) -> Result<Vec<CompressionRun>> {
    pipeline::run_batch(request)
}

pub fn app_data_dir() -> Result<PathBuf> {
    let base = dirs_fallback()?;
    let p = base.join("PIXELFORGE");
    std::fs::create_dir_all(&p)?;
    Ok(p)
}

fn dirs_fallback() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        if let Some(v) = std::env::var_os("APPDATA") {
            return Ok(PathBuf::from(v));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        if let Some(v) = std::env::var_os("XDG_DATA_HOME") {
            return Ok(PathBuf::from(v));
        }
        if let Some(v) = std::env::var_os("HOME") {
            return Ok(PathBuf::from(v).join(".local/share"));
        }
    }
    Ok(std::env::current_dir()?)
}

pub fn now_millis() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn elapsed_ms(start: Instant) -> u128 {
    start.elapsed().as_millis()
}

pub fn duration_human(d: Duration) -> String {
    let s = d.as_secs();
    format!("{:02}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
}

use pixelforge_core::{analysis, optimizer, CompressionConfig, ImageAnalysis, OutputFormat};

fn sample() -> ImageAnalysis {
    ImageAnalysis {
        width: 1920, height: 1080, aspect_ratio: 1.777, file_size: 4_000_000,
        mime_type: "image/jpeg".into(), format: "JPEG".into(), color_space: "sRGB".into(),
        channels: 3, bit_depth: 8, has_alpha: false, approximate_colors: 50000,
        entropy: 6.5, edge_density: 0.15, texture_complexity: 0.65, noise_level: 0.08,
        sharpness: 0.72, contrast: 0.55, dynamic_range: 0.62, gradient_complexity: 0.55,
        flat_region_ratio: 0.28, photo_score: 0.84, illustration_score: 0.20,
        screenshot_score: 0.10, text_density: 0.04, line_density: 0.12, detail_score: 0.72,
        transparency_ratio: 0.0, d_hash: "a".into(), a_hash: "b".into(), crypto_hash: "c".into(),
    }
}

#[test]
fn auto_prefers_photo_format() {
    let a = sample();
    let c = CompressionConfig::default();
    let candidates = optimizer::generate_candidates(&a, &c);
    assert!(!candidates.is_empty());
    assert!(matches!(candidates[0].format, OutputFormat::Webp));
}

#[test]
fn scheduler_schema_serializes() {
    let cfg = pixelforge_core::SchedulerConfig {
        timezone: "local".into(),
        windows: vec![pixelforge_core::ScheduleWindow{weekday:0,start:"09:00".into(),end:"17:00".into()}],
        queue_jobs_only_inside_windows: true,
    };
    assert!(serde_json::to_string(&cfg).is_ok());
}

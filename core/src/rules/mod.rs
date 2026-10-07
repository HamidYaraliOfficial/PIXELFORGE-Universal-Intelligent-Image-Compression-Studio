use crate::{BatchRule, ImageAnalysis};

pub fn apply_rules(a: &ImageAnalysis, rules: &[BatchRule]) -> Vec<String> {
    rules.iter().filter_map(|r| {
        if eval(&r.expression, a) { Some(r.action.clone()) } else { None }
    }).collect()
}

fn eval(expr: &str, a: &ImageAnalysis) -> bool {
    let e = expr.trim().to_ascii_lowercase();
    if let Some(rest) = e.strip_prefix("size_mb>") {
        return a.file_size as f64 / 1_048_576.0 > rest.parse::<f64>().unwrap_or(f64::MAX);
    }
    if let Some(rest) = e.strip_prefix("alpha=") {
        return a.has_alpha == (rest.trim() == "true");
    }
    if let Some(rest) = e.strip_prefix("format=") {
        return a.format.to_ascii_lowercase() == rest.trim();
    }
    if let Some(rest) = e.strip_prefix("width>") {
        return a.width > rest.parse::<u32>().unwrap_or(u32::MAX);
    }
    if let Some(rest) = e.strip_prefix("height>") {
        return a.height > rest.parse::<u32>().unwrap_or(u32::MAX);
    }
    false
}

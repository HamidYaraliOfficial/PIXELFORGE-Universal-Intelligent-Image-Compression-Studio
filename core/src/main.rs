use anyhow::Result;
use clap::{Parser, Subcommand};
use pixelforge_core::{
    analyze_file, auto_candidates, batch_compress, compress_file, BatchRequest, CompressionConfig,
    CompressionGoal, OutputFormat, SchedulerConfig,
};
use std::{path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(name="pixelforge", version, about="PIXELFORGE Universal Intelligent Image Compression Studio")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        input: PathBuf,
        #[arg(short, long)] output: PathBuf,
        #[arg(long, default_value="balanced")] goal: String,
    },
    Auto { input: PathBuf },
    Analyze { input: PathBuf },
    Batch {
        input: PathBuf,
        #[arg(short, long)] output: PathBuf,
        #[arg(long, default_value_t=true)] recursive: bool,
    },
    Target {
        input: PathBuf,
        #[arg(long)] size: String,
        #[arg(short, long)] output: PathBuf,
    },
    Convert {
        input: PathBuf,
        #[arg(short, long)] output: PathBuf,
        #[arg(long)] auto: bool,
        #[arg(long, default_value="webp")] format: String,
    },
    Benchmark { input: PathBuf, #[arg(long, default_value="balanced")] depth: String },
    Report { input: PathBuf, #[arg(long, default_value="json")] format: String, #[arg(short, long)] output: PathBuf },
    Schedule {
        #[command(subcommand)] command: ScheduleCommands
    },
}

#[derive(Subcommand)]
enum ScheduleCommands {
    Estimate {
        #[arg(long)] config: PathBuf,
        #[arg(long, default_value_t=600)] job_seconds: u64,
    }
}

fn parse_goal(s: &str) -> CompressionGoal {
    match s {
        "maximum" => CompressionGoal::MaximumCompression,
        "quality" => CompressionGoal::MaximumQuality,
        "web" => CompressionGoal::WebOptimized,
        _ => CompressionGoal::Balanced,
    }
}

fn parse_format(s: &str) -> OutputFormat {
    match s.to_lowercase().as_str() {
        "jpeg" | "jpg" => OutputFormat::Jpeg,
        "png" => OutputFormat::Png,
        "webp" => OutputFormat::Webp,
        "avif" => OutputFormat::Avif,
        "jxl" => OutputFormat::Jxl,
        "tiff" | "tif" => OutputFormat::Tiff,
        "bmp" => OutputFormat::Bmp,
        "gif" => OutputFormat::Gif,
        "heif" | "heic" => OutputFormat::Heif,
        "pnm" => OutputFormat::Pnm,
        _ => OutputFormat::Auto,
    }
}

fn parse_size(s: &str) -> Result<u64> {
    let t = s.trim().to_ascii_uppercase();
    let (n, m) = if let Some(v) = t.strip_suffix("KB") { (v, 1024f64) }
        else if let Some(v) = t.strip_suffix("MB") { (v, 1024f64*1024.0) }
        else { (t.as_str(), 1.0) };
    Ok((n.parse::<f64>()? * m) as u64)
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Analyze { input } => println!("{}", serde_json::to_string_pretty(&analyze_file(&input)?)?),
        Commands::Auto { input } => {
            let cfg = CompressionConfig::default();
            println!("{}", serde_json::to_string_pretty(&auto_candidates(&input, &cfg)?)?);
        }
        Commands::Compress { input, output, goal } => {
            let mut cfg = CompressionConfig::default();
            cfg.goal = parse_goal(&goal);
            let r = compress_file(&input, &output, &cfg)?;
            println!("{}", serde_json::to_string_pretty(&r)?);
        }
        Commands::Convert { input, output, auto, format } => {
            let mut cfg = CompressionConfig::default();
            cfg.output_format = if auto { OutputFormat::Auto } else { parse_format(&format) };
            let r = compress_file(&input, &output, &cfg)?;
            println!("{}", serde_json::to_string_pretty(&r)?);
        }
        Commands::Target { input, size, output } => {
            let mut cfg = CompressionConfig::default();
            cfg.goal = CompressionGoal::TargetFileSize { bytes: parse_size(&size)? };
            let r = compress_file(&input, &output, &cfg)?;
            println!("{}", serde_json::to_string_pretty(&r)?);
        }
        Commands::Batch { input, output, recursive } => {
            let req = BatchRequest {
                input, output_dir: output, config: CompressionConfig::default(),
                recursive, rules: vec![], schedule: None
            };
            let r = batch_compress(&req)?;
            println!("{}", serde_json::to_string_pretty(&r)?);
        }
        Commands::Benchmark { input, depth } => {
            let mut cfg = CompressionConfig::default();
            cfg.auto_depth = match depth.as_str() {
                "fast" => pixelforge_core::AutoDepth::FastAuto,
                "deep" => pixelforge_core::AutoDepth::DeepAuto,
                _ => pixelforge_core::AutoDepth::BalancedAuto,
            };
            let a = analyze_file(&input)?;
            let c = auto_candidates(&input, &cfg)?;
            println!("{}", serde_json::json!({"analysis":a,"candidates":c}));
        }
        Commands::Report { input, format, output } => {
            let text = std::fs::read_to_string(&input)?;
            let v: Vec<pixelforge_core::CompressionRun> = serde_json::from_str(&text)?;
            match format.as_str() {
                "csv" => pixelforge_core::report::write_csv(&v, &output)?,
                "html" => pixelforge_core::report::write_html(&v, &output)?,
                _ => pixelforge_core::report::write_json(&v, &output)?,
            }
        }
        Commands::Schedule { command } => match command {
            ScheduleCommands::Estimate { config, job_seconds } => {
                let cfg: SchedulerConfig = serde_json::from_str(&std::fs::read_to_string(config)?)?;
                println!("{}", serde_json::to_string_pretty(&pixelforge_core::scheduler::estimate(&cfg, job_seconds)?)?);
            }
        }
    }
    Ok(())
}

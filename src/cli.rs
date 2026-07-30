use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "ferrum-scan",
    author,
    version,
    about = "Ultra-fast multithreaded repository scanner and health inspector written in pure Rust",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Target directory to scan (default: current directory)
    #[arg(default_value = ".", global = true)]
    pub path: PathBuf,

    /// Output format (text, md, json)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text, global = true)]
    pub format: OutputFormat,

    /// Output file path for export (if saving to file)
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,

    /// Large file threshold in MB
    #[arg(long, default_value_t = 5.0, global = true)]
    pub jumbo_threshold_mb: f64,

    /// Perform a full scan including hidden files and ignored paths (.gitignore)
    #[arg(long, global = true)]
    pub full: bool,

    /// Inspect top directory storage usage breakdown
    #[arg(short, long, global = true)]
    pub storage: bool,

    /// Detect duplicate files across the repository
    #[arg(short, long, global = true)]
    pub duplicates: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Export scan report to a file (Markdown / JSON)
    Export,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OutputFormat {
    Text,
    Md,
    Json,
}

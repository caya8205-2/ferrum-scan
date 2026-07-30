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
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,

    /// Output format (text, md, json)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Output file path for export (if saving to file)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Large file threshold in MB
    #[arg(long, default_value_t = 5.0)]
    pub jumbo_threshold_mb: f64,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan target repository and display results in terminal
    Scan {
        /// Target directory path
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Large file threshold in MB
        #[arg(long, default_value_t = 5.0)]
        jumbo_threshold_mb: f64,
    },
    /// Scan target repository and export report to a file
    Export {
        /// Target directory path
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Export format (md, json)
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Md)]
        format: OutputFormat,

        /// Output file path (default: SCAN_REPORT.md or scan_report.json)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Large file threshold in MB
        #[arg(long, default_value_t = 5.0)]
        jumbo_threshold_mb: f64,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OutputFormat {
    Text,
    Md,
    Json,
}

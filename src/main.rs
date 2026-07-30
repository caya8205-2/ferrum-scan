mod cli;
mod display;
mod scanner;

use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use display::{export_report, print_terminal_report};
use scanner::walker::scan_repository;
use std::process;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Scan {
            path,
            jumbo_threshold_mb,
        }) => {
            let stats = scan_repository(&path, jumbo_threshold_mb);
            print_terminal_report(&stats);
        }
        Some(Commands::Export {
            path,
            format,
            output,
            jumbo_threshold_mb,
        }) => {
            let stats = scan_repository(&path, jumbo_threshold_mb);
            print_terminal_report(&stats);
            match export_report(&stats, format, output.as_deref()) {
                Ok(file_path) => {
                    println!("✅ Successfully exported report to: {}", file_path.display());
                }
                Err(err) => {
                    eprintln!("❌ Failed to export report: {}", err);
                    process::exit(1);
                }
            }
        }
        None => {
            // Default behavior if no subcommand is provided
            let stats = scan_repository(&cli.path, cli.jumbo_threshold_mb);
            print_terminal_report(&stats);

            if cli.output.is_some() || cli.format != OutputFormat::Text {
                match export_report(&stats, cli.format, cli.output.as_deref()) {
                    Ok(file_path) => {
                        println!("✅ Successfully exported report to: {}", file_path.display());
                    }
                    Err(err) => {
                        eprintln!("❌ Failed to export report: {}", err);
                        process::exit(1);
                    }
                }
            }
        }
    }
}

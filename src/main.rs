mod cli;
mod display;
mod scanner;
mod uninstaller;

use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use display::{export_report, print_terminal_report};
use scanner::walker::scan_repository;
use std::process;
use uninstaller::handle_uninstall;

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Uninstall { yes }) = cli.command {
        handle_uninstall(yes);
        return;
    }

    let stats = scan_repository(
        &cli.path,
        cli.jumbo_threshold_mb,
        cli.full,
        cli.storage,
        cli.duplicates,
    );

    print_terminal_report(&stats);

    let is_export_subcommand = matches!(cli.command, Some(Commands::Export));

    if is_export_subcommand || cli.output.is_some() || cli.format != OutputFormat::Text {
        let export_format = if is_export_subcommand && cli.format == OutputFormat::Text {
            OutputFormat::Md
        } else {
            cli.format
        };

        match export_report(&stats, export_format, cli.output.as_deref()) {
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

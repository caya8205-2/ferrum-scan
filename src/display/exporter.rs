use crate::cli::OutputFormat;
use crate::scanner::RepoStats;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn export_report(
    stats: &RepoStats,
    format: OutputFormat,
    output_path: Option<&Path>,
) -> Result<PathBuf, String> {
    let default_name = match format {
        OutputFormat::Md => "SCAN_REPORT.md",
        OutputFormat::Json => "scan_report.json",
        OutputFormat::Text => "scan_report.txt",
    };

    let target_path = match output_path {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from(default_name),
    };

    let content = match format {
        OutputFormat::Md => generate_markdown(stats),
        OutputFormat::Json => {
            serde_json::to_string_pretty(stats).map_err(|e| e.to_string())?
        }
        OutputFormat::Text => generate_text(stats),
    };

    let mut file = File::create(&target_path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;

    Ok(target_path)
}

fn generate_markdown(stats: &RepoStats) -> String {
    let mut md = String::new();
    let size_mb = stats.total_bytes as f64 / (1024.0 * 1024.0);

    md.push_str("# ⚡ Ferrum-Scan Repository Report\n\n");
    md.push_str(&format!("- **Scan Date**: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    md.push_str(&format!("- **Root Directory**: `{}`\n", stats.root_path.display()));
    md.push_str(&format!("- **Total Files**: `{}`\n", stats.total_files));
    md.push_str(&format!("- **Total Size**: `{:.2} MB` ({} bytes)\n", size_mb, stats.total_bytes));
    md.push_str(&format!("- **Scan Duration**: `{} ms`\n\n", stats.scan_duration_ms));

    md.push_str("## 📊 Language Breakdown\n\n");
    md.push_str("| Language | Files | Code LOC | Blank | Comment | Total LOC | Share % |\n");
    md.push_str("| :--- | :---: | :---: | :---: | :---: | :---: | :---: |\n");

    for lang in &stats.languages {
        let share = if stats.total_code_lines > 0 {
            (lang.code_lines as f64 / stats.total_code_lines as f64) * 100.0
        } else {
            0.0
        };
        md.push_str(&format!(
            "| **{}** | {} | {} | {} | {} | {} | {:.1}% |\n",
            lang.name,
            lang.files,
            lang.code_lines,
            lang.blank_lines,
            lang.comment_lines,
            lang.total_lines,
            share
        ));
    }
    md.push_str(&format!(
        "| **TOTAL** | **{}** | **{}** | **{}** | **{}** | **{}** | **100%** |\n\n",
        stats.total_files,
        stats.total_code_lines,
        stats.total_blank_lines,
        stats.total_comment_lines,
        stats.total_lines
    ));

    if let Some(ref folders) = stats.top_folders {
        md.push_str("## 📂 Storage Breakdown (Top Folders)\n\n");
        md.push_str("| Folder Name | Files | Size (MB) |\n");
        md.push_str("| :--- | :---: | :---: |\n");
        for folder in folders {
            let folder_mb = folder.bytes as f64 / (1024.0 * 1024.0);
            md.push_str(&format!(
                "| `{}` | {} | {:.2} MB |\n",
                folder.relative_path, folder.files_count, folder_mb
            ));
        }
        md.push('\n');
    }

    if let Some(ref dups) = stats.duplicate_groups {
        md.push_str("## 👯 Duplicate Files Detector\n\n");
        let wasted_mb = stats.total_wasted_bytes as f64 / (1024.0 * 1024.0);
        md.push_str(&format!("- **Total Duplicate Groups**: `{}`\n", dups.len()));
        md.push_str(&format!("- **Wasted Storage Space**: `{:.2} MB`\n\n", wasted_mb));

        if !dups.is_empty() {
            md.push_str("| Size / File | Copies | Paths |\n");
            md.push_str("| :---: | :---: | :--- |\n");
            for group in dups.iter().take(15) {
                let size_mb = group.size_bytes as f64 / (1024.0 * 1024.0);
                let paths_str = group
                    .files
                    .iter()
                    .map(|p| format!("`{}`", p.display()))
                    .collect::<Vec<_>>()
                    .join("<br>");

                md.push_str(&format!(
                    "| {:.2} MB | {} | {} |\n",
                    size_mb,
                    group.files.len(),
                    paths_str
                ));
            }
            md.push('\n');
        }
    }

    md.push_str("## 🩺 Git Health Inspector\n\n");
    if stats.git_health.is_git_repo {
        md.push_str("- **Git Repo**: Yes\n");
        md.push_str(&format!("- **Total Commits**: `{}`\n", stats.git_health.commit_count));
        md.push_str(&format!("- **Contributors**: `{}`\n", stats.git_health.total_contributors));
        if let Some(ref date) = stats.git_health.last_commit_date {
            md.push_str(&format!("- **Last Commit**: `{}`\n", date));
        }

        if !stats.git_health.top_contributors.is_empty() {
            md.push_str("\n### Top Contributors\n\n");
            md.push_str("| Contributor | Commits |\n");
            md.push_str("| :--- | :---: |\n");
            for (name, commits) in &stats.git_health.top_contributors {
                md.push_str(&format!("| {} | {} |\n", name, commits));
            }
        }
        md.push('\n');
    } else {
        md.push_str("⚠️ Not a git repository.\n\n");
    }

    if !stats.jumbo_files.is_empty() {
        md.push_str("## 🚨 Jumbo Files (> threshold)\n\n");
        md.push_str("| File Path | Size (MB) |\n");
        md.push_str("| :--- | :---: |\n");
        for j in &stats.jumbo_files {
            md.push_str(&format!("| `{}` | {:.2} MB |\n", j.path.display(), j.size_mb));
        }
        md.push('\n');
    }

    md.push_str("## 💡 Repository Insights\n\n");
    for insight in &stats.insights {
        md.push_str(&format!("- {}\n", insight));
    }
    md.push('\n');

    md
}

fn generate_text(stats: &RepoStats) -> String {
    format!(
        "Ferrum-Scan Report\nTarget: {}\nTotal Files: {}\nTotal LOC: {}\nScan Time: {} ms\n",
        stats.root_path.display(),
        stats.total_files,
        stats.total_lines,
        stats.scan_duration_ms
    )
}

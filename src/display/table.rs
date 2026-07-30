use crate::scanner::RepoStats;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};

pub fn print_terminal_report(stats: &RepoStats) {
    println!("\n⚡ FERRUM-SCAN REPOSITORY REPORT ⚡\n");

    // General Summary
    let size_mb = stats.total_bytes as f64 / (1024.0 * 1024.0);
    println!("📌 Path              : {}", stats.root_path.display());
    println!("📁 Total Files        : {}", stats.total_files);
    println!("💾 Total Size         : {:.2} MB ({} bytes)", size_mb, stats.total_bytes);
    println!("⏱️  Scan Time         : {} ms", stats.scan_duration_ms);
    println!("📝 Total Lines        : {} (Code: {}, Blank: {}, Comments: {})\n", 
        stats.total_lines, stats.total_code_lines, stats.total_blank_lines, stats.total_comment_lines);

    // Language Breakdown Table
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Language").fg(Color::Cyan),
            Cell::new("Files").fg(Color::Cyan),
            Cell::new("Code LOC").fg(Color::Green),
            Cell::new("Blank").fg(Color::Yellow),
            Cell::new("Comment").fg(Color::Blue),
            Cell::new("Total LOC").fg(Color::White),
            Cell::new("Share %").fg(Color::Magenta),
        ]);

    for lang in &stats.languages {
        let share = if stats.total_code_lines > 0 {
            (lang.code_lines as f64 / stats.total_code_lines as f64) * 100.0
        } else {
            0.0
        };

        table.add_row(vec![
            Cell::new(&lang.name).fg(Color::Cyan),
            Cell::new(lang.files),
            Cell::new(lang.code_lines).fg(Color::Green),
            Cell::new(lang.blank_lines),
            Cell::new(lang.comment_lines),
            Cell::new(lang.total_lines),
            Cell::new(format!("{:.1}%", share)).fg(Color::Magenta),
        ]);
    }

    println!("--- LANGUAGE BREAKDOWN ---");
    println!("{}\n", table);

    // Storage Breakdown Section
    if let Some(ref folders) = stats.top_folders {
        println!("--- TOP DIRECTORIES BY STORAGE (--storage) ---");
        if folders.is_empty() {
            println!("No subdirectories found.\n");
        } else {
            let mut storage_table = Table::new();
            storage_table
                .load_preset(UTF8_FULL)
                .set_header(vec![
                    Cell::new("Folder Name").fg(Color::Yellow),
                    Cell::new("Files").fg(Color::Cyan),
                    Cell::new("Size (MB)").fg(Color::Green),
                ]);

            for folder in folders {
                let folder_mb = folder.bytes as f64 / (1024.0 * 1024.0);
                storage_table.add_row(vec![
                    Cell::new(&folder.relative_path).fg(Color::Yellow),
                    Cell::new(folder.files_count),
                    Cell::new(format!("{:.2} MB", folder_mb)).fg(Color::Green),
                ]);
            }
            println!("{}\n", storage_table);
        }
    }

    // Duplicate Files Section
    if let Some(ref dups) = stats.duplicate_groups {
        println!("--- DUPLICATE FILES DETECTOR (--duplicates) ---");
        let wasted_mb = stats.total_wasted_bytes as f64 / (1024.0 * 1024.0);
        println!("⚠️  Total Wasted Storage Space: {:.2} MB ({} duplicate groups)\n", wasted_mb, dups.len());

        if dups.is_empty() {
            println!("✅ No duplicate files found.\n");
        } else {
            let mut dup_table = Table::new();
            dup_table
                .load_preset(UTF8_FULL)
                .set_header(vec![
                    Cell::new("Group Hash").fg(Color::Magenta),
                    Cell::new("Size/File").fg(Color::Cyan),
                    Cell::new("Copies").fg(Color::Yellow),
                    Cell::new("Duplicate Paths").fg(Color::White),
                ]);

            for group in dups.iter().take(10) {
                let size_mb = group.size_bytes as f64 / (1024.0 * 1024.0);
                let paths_str = group
                    .files
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                dup_table.add_row(vec![
                    Cell::new(format!("{:x}", group.hash)).fg(Color::Magenta),
                    Cell::new(format!("{:.2} MB", size_mb)).fg(Color::Cyan),
                    Cell::new(group.files.len()).fg(Color::Yellow),
                    Cell::new(paths_str),
                ]);
            }
            println!("{}\n", dup_table);
        }
    }

    // Git Health Section
    if stats.git_health.is_git_repo {
        println!("--- GIT HEALTH INSPECTOR ---");
        println!("✅ Git Repository Detected");
        println!("📊 Total Commits        : {}", stats.git_health.commit_count);
        println!("👥 Total Contributors   : {}", stats.git_health.total_contributors);
        if let Some(ref date) = stats.git_health.last_commit_date {
            println!("📅 Last Commit Date     : {}", date);
        }

        if !stats.git_health.top_contributors.is_empty() {
            println!("\n🏆 Top Contributors:");
            for (name, commits) in &stats.git_health.top_contributors {
                println!("   • {:<20} : {} commits", name, commits);
            }
        }
        println!();
    } else {
        println!("--- GIT HEALTH INSPECTOR ---");
        println!("⚠️  Not a git repository (or .git folder not found)\n");
    }

    // Jumbo Files Warning
    if !stats.jumbo_files.is_empty() {
        println!("--- 🚨 JUMBO FILES DETECTED (> threshold) ---");
        let mut jumbo_table = Table::new();
        jumbo_table
            .load_preset(UTF8_FULL)
            .set_header(vec![
                Cell::new("File Path").fg(Color::Red),
                Cell::new("Size (MB)").fg(Color::Red),
            ]);

        for j in &stats.jumbo_files {
            jumbo_table.add_row(vec![
                Cell::new(j.path.display().to_string()),
                Cell::new(format!("{:.2} MB", j.size_mb)),
            ]);
        }
        println!("{}\n", jumbo_table);
    }

    // Repository Insights (FINAL SECTION)
    println!("--- 💡 REPOSITORY INSIGHTS ---");
    for insight in &stats.insights {
        println!("• {}", insight);
    }
    println!();
}

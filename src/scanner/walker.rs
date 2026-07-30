use super::git_inspector::inspect_git;
use super::metrics::analyze_file;
use super::{JumboFile, LanguageStat, RepoStats};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

pub fn scan_repository(repo_path: &Path, jumbo_threshold_mb: f64) -> RepoStats {
    let start_time = Instant::now();

    // Build directory walker respecting .gitignore and ignoring hidden files
    let walker = WalkBuilder::new(repo_path)
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    let mut file_paths = Vec::new();
    for entry in walker.flatten() {
        let path = entry.path();
        if path.is_file() {
            file_paths.push(path.to_path_buf());
        }
    }

    let jumbo_bytes_threshold = (jumbo_threshold_mb * 1024.0 * 1024.0) as u64;

    // Parallel analysis of files using Rayon
    let file_results: Vec<_> = file_paths
        .par_iter()
        .filter_map(|path| {
            let metrics = analyze_file(path)?;
            Some((path.clone(), metrics))
        })
        .collect();

    let total_files = file_results.len();
    let mut total_bytes = 0u64;
    let mut lang_map: HashMap<String, LanguageStat> = HashMap::new();
    let mut jumbo_files = Vec::new();

    let mut total_code_lines = 0;
    let mut total_blank_lines = 0;
    let mut total_comment_lines = 0;

    for (path, metrics) in file_results {
        total_bytes += metrics.bytes;
        let total_lines = metrics.code_lines + metrics.blank_lines + metrics.comment_lines;

        total_code_lines += metrics.code_lines;
        total_blank_lines += metrics.blank_lines;
        total_comment_lines += metrics.comment_lines;

        if metrics.bytes >= jumbo_bytes_threshold {
            let size_mb = metrics.bytes as f64 / (1024.0 * 1024.0);
            jumbo_files.push(JumboFile {
                path: path.clone(),
                size_mb,
            });
        }

        let entry = lang_map
            .entry(metrics.language.clone())
            .or_insert_with(|| LanguageStat {
                name: metrics.language.clone(),
                files: 0,
                code_lines: 0,
                blank_lines: 0,
                comment_lines: 0,
                total_lines: 0,
                bytes: 0,
            });

        entry.files += 1;
        entry.code_lines += metrics.code_lines;
        entry.blank_lines += metrics.blank_lines;
        entry.comment_lines += metrics.comment_lines;
        entry.total_lines += total_lines;
        entry.bytes += metrics.bytes;
    }

    let mut languages: Vec<LanguageStat> = lang_map.into_values().collect();
    languages.sort_by(|a, b| b.code_lines.cmp(&a.code_lines));

    let git_health = inspect_git(repo_path);
    let scan_duration_ms = start_time.elapsed().as_millis();
    let total_lines = total_code_lines + total_blank_lines + total_comment_lines;

    RepoStats {
        root_path: repo_path.to_path_buf(),
        total_files,
        total_bytes,
        scan_duration_ms,
        languages,
        jumbo_files,
        git_health,
        total_code_lines,
        total_blank_lines,
        total_comment_lines,
        total_lines,
    }
}

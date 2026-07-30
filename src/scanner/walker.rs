use super::duplicates::find_duplicates;
use super::git_inspector::inspect_git;
use super::insights::generate_insights;
use super::metrics::analyze_file;
use super::storage::analyze_storage;
use super::{JumboFile, LanguageStat, RepoStats};
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

pub fn scan_repository(
    repo_path: &Path,
    jumbo_threshold_mb: f64,
    full_scan: bool,
    include_storage: bool,
    include_duplicates: bool,
) -> RepoStats {
    let start_time = Instant::now();

    let mut builder = WalkBuilder::new(repo_path);

    if full_scan {
        builder
            .hidden(false)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false)
            .ignore(false);
    } else {
        builder
            .hidden(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .ignore(true);

        // Fallback filter for common heavy build/dependency directories
        let mut override_builder = OverrideBuilder::new(repo_path);
        let _ = override_builder.add("!**/node_modules/**");
        let _ = override_builder.add("!**/target/**");
        let _ = override_builder.add("!**/.git/**");
        let _ = override_builder.add("!**/dist/**");
        let _ = override_builder.add("!**/.next/**");
        let _ = override_builder.add("!**/vendor/**");
        if let Ok(overrides) = override_builder.build() {
            builder.overrides(overrides);
        }
    }

    let walker = builder.build();

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

    for (path, metrics) in file_results.iter() {
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

    let file_sizes: Vec<_> = file_results
        .into_iter()
        .map(|(path, metrics)| (path, metrics.bytes))
        .collect();

    let top_folders = if include_storage {
        Some(analyze_storage(&file_sizes, repo_path))
    } else {
        None
    };

    let (duplicate_groups, total_wasted_bytes) = if include_duplicates {
        let (dups, wasted) = find_duplicates(&file_sizes);
        (Some(dups), wasted)
    } else {
        (None, 0)
    };

    let mut languages: Vec<LanguageStat> = lang_map.into_values().collect();
    languages.sort_by(|a, b| b.code_lines.cmp(&a.code_lines));

    let git_health = inspect_git(repo_path);
    let scan_duration_ms = start_time.elapsed().as_millis();
    let total_lines = total_code_lines + total_blank_lines + total_comment_lines;

    let mut partial_stats = RepoStats {
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
        top_folders,
        duplicate_groups,
        total_wasted_bytes,
        insights: Vec::new(),
    };

    partial_stats.insights = generate_insights(&partial_stats);
    partial_stats
}

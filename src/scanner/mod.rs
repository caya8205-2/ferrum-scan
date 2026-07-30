pub mod duplicates;
pub mod git_inspector;
pub mod insights;
pub mod metrics;
pub mod storage;
pub mod walker;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageStat {
    pub name: String,
    pub files: usize,
    pub code_lines: usize,
    pub blank_lines: usize,
    pub comment_lines: usize,
    pub total_lines: usize,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumboFile {
    pub path: PathBuf,
    pub size_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitHealth {
    pub is_git_repo: bool,
    pub commit_count: usize,
    pub total_contributors: usize,
    pub active_contributors: usize,
    pub top_contributors: Vec<(String, usize)>,
    pub last_commit_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderStat {
    pub relative_path: String,
    pub bytes: u64,
    pub files_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub size_bytes: u64,
    pub hash: u64,
    pub files: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoStats {
    pub root_path: PathBuf,
    pub total_files: usize,
    pub total_bytes: u64,
    pub scan_duration_ms: u128,
    pub languages: Vec<LanguageStat>,
    pub jumbo_files: Vec<JumboFile>,
    pub git_health: GitHealth,
    pub total_code_lines: usize,
    pub total_blank_lines: usize,
    pub total_comment_lines: usize,
    pub total_lines: usize,
    pub top_folders: Option<Vec<FolderStat>>,
    pub duplicate_groups: Option<Vec<DuplicateGroup>>,
    pub total_wasted_bytes: u64,
    pub insights: Vec<String>,
}

use super::GitHealth;
use std::path::Path;
use std::process::Command;

pub fn inspect_git(repo_path: &Path) -> GitHealth {
    let git_dir = repo_path.join(".git");
    if !git_dir.exists() {
        return GitHealth::default();
    }

    let commit_count = get_commit_count(repo_path);
    let contributors = get_contributors(repo_path);
    let last_commit_date = get_last_commit_date(repo_path);

    let total_contributors = contributors.len();
    let top_contributors = contributors.into_iter().take(5).collect();

    GitHealth {
        is_git_repo: true,
        commit_count,
        total_contributors,
        active_contributors: total_contributors,
        top_contributors,
        last_commit_date,
    }
}

fn get_commit_count(repo_path: &Path) -> usize {
    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg("HEAD")
        .current_dir(repo_path)
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout);
            return s.trim().parse::<usize>().unwrap_or(0);
        }
    }
    0
}

fn get_contributors(repo_path: &Path) -> Vec<(String, usize)> {
    let output = Command::new("git")
        .arg("shortlog")
        .arg("-sn")
        .arg("--all")
        .current_dir(repo_path)
        .output();

    let mut result = Vec::new();
    if let Ok(out) = output {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines() {
                let trimmed = line.trim();
                if let Some((count_str, name)) = trimmed.split_once('\t') {
                    if let Ok(count) = count_str.trim().parse::<usize>() {
                        result.push((name.trim().to_string(), count));
                    }
                }
            }
        }
    }
    result
}

fn get_last_commit_date(repo_path: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--format=%cd")
        .arg("--date=short")
        .current_dir(repo_path)
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }
    None
}

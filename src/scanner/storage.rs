use super::FolderStat;
use std::collections::HashMap;
use std::path::{Component, Path};

pub fn analyze_storage(files: &[(std::path::PathBuf, u64)], root_path: &Path) -> Vec<FolderStat> {
    let mut folder_map: HashMap<String, (u64, usize)> = HashMap::new();

    // Canonicalize or clean root path reference for prefix stripping
    let clean_root = if root_path.as_os_str() == "." {
        Path::new("")
    } else {
        root_path
    };

    for (path, bytes) in files {
        let rel_path = match path.strip_prefix(clean_root) {
            Ok(p) => p,
            Err(_) => path.as_path(),
        };

        let mut normal_components = rel_path.components().filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().to_string()),
            _ => None,
        });

        if let Some(top_dir) = normal_components.next() {
            if normal_components.next().is_some() {
                // Inside a top-level directory
                let entry = folder_map.entry(top_dir).or_insert((0, 0));
                entry.0 += bytes;
                entry.1 += 1;
            } else {
                // File directly in root directory
                let entry = folder_map.entry("<root files>".to_string()).or_insert((0, 0));
                entry.0 += bytes;
                entry.1 += 1;
            }
        }
    }

    let mut folders: Vec<FolderStat> = folder_map
        .into_iter()
        .map(|(relative_path, (bytes, files_count))| FolderStat {
            relative_path,
            bytes,
            files_count,
        })
        .collect();

    folders.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    folders.truncate(10);

    folders
}

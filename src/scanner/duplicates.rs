use super::DuplicateGroup;
use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn find_duplicates(files: &[(PathBuf, u64)]) -> (Vec<DuplicateGroup>, u64) {
    // Step 1: Group files by byte size
    let mut size_groups: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for (path, size) in files {
        if *size > 0 {
            size_groups.entry(*size).or_default().push(path.clone());
        }
    }

    // Filter candidate groups with > 1 file
    let candidate_files: Vec<(u64, PathBuf)> = size_groups
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .flat_map(|(size, paths)| paths.into_iter().map(move |p| (size, p)))
        .collect();

    if candidate_files.is_empty() {
        return (Vec::new(), 0);
    }

    // Step 2: Parallel hashing of candidate files
    let hashed_files: Vec<_> = candidate_files
        .par_iter()
        .filter_map(|(size, path)| {
            let hash = hash_file_content(path)?;
            Some((*size, hash, path.clone()))
        })
        .collect();

    // Step 3: Group by (size, hash)
    let mut hash_groups: HashMap<(u64, u64), Vec<PathBuf>> = HashMap::new();
    for (size, hash, path) in hashed_files {
        hash_groups.entry((size, hash)).or_default().push(path);
    }

    let mut duplicate_groups = Vec::new();
    let mut total_wasted_bytes = 0u64;

    for ((size_bytes, hash), paths) in hash_groups {
        if paths.len() > 1 {
            let wasted = (paths.len() - 1) as u64 * size_bytes;
            total_wasted_bytes += wasted;
            duplicate_groups.push(DuplicateGroup {
                size_bytes,
                hash,
                files: paths,
            });
        }
    }

    duplicate_groups.sort_by(|a, b| {
        let wasted_a = (a.files.len() - 1) as u64 * a.size_bytes;
        let wasted_b = (b.files.len() - 1) as u64 * b.size_bytes;
        wasted_b.cmp(&wasted_a)
    });

    (duplicate_groups, total_wasted_bytes)
}

fn hash_file_content(path: &PathBuf) -> Option<u64> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut hasher = DefaultHasher::new();
    let mut buffer = [0u8; 65536]; // 64KB buffer

    loop {
        let count = match reader.read(&mut buffer) {
            Ok(c) => c,
            Err(_) => return None,
        };
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }

    Some(hasher.finish())
}

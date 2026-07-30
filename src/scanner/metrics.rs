use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct SingleFileMetrics {
    pub language: String,
    pub bytes: u64,
    pub code_lines: usize,
    pub blank_lines: usize,
    pub comment_lines: usize,
}

pub fn detect_language(path: &Path) -> String {
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    match filename {
        "Dockerfile" | "dockerfile" => return "Dockerfile".to_string(),
        "Makefile" | "makefile" => return "Makefile".to_string(),
        "Cargo.toml" | "Cargo.lock" => return "Rust".to_string(),
        "package.json" | "tsconfig.json" => return "JSON".to_string(),
        _ => {}
    }

    match ext.as_str() {
        "rs" => "Rust",
        "js" | "mjs" | "cjs" => "JavaScript",
        "ts" | "mts" | "cts" => "TypeScript",
        "jsx" => "JavaScript (JSX)",
        "tsx" => "TypeScript (TSX)",
        "py" => "Python",
        "go" => "Go",
        "c" | "h" => "C",
        "cpp" | "hpp" | "cc" | "cxx" => "C++",
        "java" => "Java",
        "kt" | "kts" => "Kotlin",
        "swift" => "Swift",
        "cs" => "C#",
        "html" | "htm" => "HTML",
        "css" | "scss" | "sass" => "CSS",
        "svelte" => "Svelte",
        "vue" => "Vue",
        "json" => "JSON",
        "toml" => "TOML",
        "yaml" | "yml" => "YAML",
        "md" | "markdown" => "Markdown",
        "sh" | "bash" | "zsh" | "ps1" => "Shell Script",
        "sql" => "SQL",
        "lua" => "Lua",
        "hs" => "Haskell",
        "rb" => "Ruby",
        "php" => "PHP",
        "dart" => "Dart",
        "zig" => "Zig",
        "xml" => "XML",
        _ => "Other",
    }.to_string()
}

pub fn analyze_file(path: &Path) -> Option<SingleFileMetrics> {
    let metadata = std::fs::metadata(path).ok()?;
    if !metadata.is_file() {
        return None;
    }

    let bytes = metadata.len();
    let language = detect_language(path);

    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut code_lines = 0;
    let mut blank_lines = 0;
    let mut comment_lines = 0;

    let comment_prefix = match language.as_str() {
        "Rust" | "JavaScript" | "TypeScript" | "JavaScript (JSX)" | "TypeScript (TSX)"
        | "Go" | "C" | "C++" | "Java" | "Kotlin" | "Swift" | "C#" | "Svelte" | "Vue" | "Dart"
        | "Zig" | "PHP" => "//",
        "Python" | "Shell Script" | "Ruby" | "YAML" | "TOML" | "Dockerfile" | "Makefile" => "#",
        "SQL" | "Lua" | "Haskell" => "--",
        _ => "//",
    };

    let mut in_block_comment = false;

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => return None, // Non-UTF8 / binary file skip
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            blank_lines += 1;
            continue;
        }

        // Simple block comment check for C-style
        if in_block_comment {
            comment_lines += 1;
            if trimmed.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }

        if trimmed.starts_with("/*") {
            comment_lines += 1;
            if !trimmed.contains("*/") {
                in_block_comment = true;
            }
            continue;
        }

        if trimmed.starts_with(comment_prefix) {
            comment_lines += 1;
        } else {
            code_lines += 1;
        }
    }

    Some(SingleFileMetrics {
        language,
        bytes,
        code_lines,
        blank_lines,
        comment_lines,
    })
}

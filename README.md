# ferrum-scan

[![Rust](https://img.shields.io/badge/Rust-2021-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![CLI](https://img.shields.io/badge/CLI-Clap_4.5-blue?style=for-the-badge)](https://docs.rs/clap/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)

**ferrum-scan** is an ultra-fast, multithreaded repository scanner and health inspector CLI written in pure Rust.

## Features

- **Ultra-Fast Repository Scanner**: Scans project directories in parallel within milliseconds using `rayon` and `ignore`. Respects `.gitignore`, `.ignore`, and automatically skips `.git` and `target` directories.
- **Language Breakdown & Metrics**: Computes exact Lines of Code (LOC), blank lines, and comment lines per programming language.
- **Git Health Inspector**: Analyzes total commits, active contributors, top contributors, and last commit dates.
- **Jumbo File Detector**: Flags large files exceeding a customizable threshold (default 5MB) to help keep repositories lean.
- **Terminal Visuals & Export**: Formats statistics in clean terminal tables using `comfy-table` and supports exporting reports to `Markdown` (`SCAN_REPORT.md`) or `JSON`.

## Installation & Build

Ensure Rust and Cargo are installed on your system.

### Build from Source
```bash
cargo build --release
```

### Global Installation
```bash
cargo install --path .
```

## Usage

### Scan Current Directory
```bash
ferrum-scan
```

### Scan Specific Path
```bash
ferrum-scan scan /path/to/repository
```

### Export Report to Markdown
```bash
ferrum-scan export --format md --output SCAN_REPORT.md
```

### Export Report to JSON
```bash
ferrum-scan export --format json --output report.json
```

## License

This project is licensed under the MIT License.

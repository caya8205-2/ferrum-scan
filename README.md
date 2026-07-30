<p align="center">
  <img src="public/ascii.svg" alt="Ferrum-scan ASCII" width="450" />
</p>

<p align="center"><b>Ultra-fast multithreaded repository scanner and health inspector CLI tools built in Rust</b></p>

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-2021-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![CLI](https://img.shields.io/badge/CLI-Clap_4.5-blue?style=for-the-badge)](https://docs.rs/clap/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)

</div>

<p align="center">
  <img src="./public/demo.png" width="900">
</p>

## Quick 1-Liner Installation (No Rust / Cargo Required)

Install binary directly from GitHub Releases without needing Rust toolchains or Visual Studio Build Tools:

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/caya8205-2/ferrum-scan/main/install.ps1 | iex
```

### Linux / macOS (Bash)
```bash
curl -fsSL https://raw.githubusercontent.com/caya8205-2/ferrum-scan/main/install.sh | bash
```

---

## Features

- **Ultra-Fast Repository Scanner**: Scans project directories in parallel within milliseconds using `rayon` and `ignore`. Respects `.gitignore`, `.ignore`, and automatically skips `.git`, `node_modules`, `target`, and hidden directories by default.
- **Repository Insights & Health Rating**: Evaluates overall repository health, documentation ratio, Git status, and assigns an overall health score (e.g., `A+ (Optimal & Clean)`).
- **Storage Breakdown (`--storage`, `-s`)**: Analyzes disk space consumption across top subdirectories to identify storage hogs.
- **Duplicate Files Detector (`--duplicates`, `-d`)**: Finds identical duplicate files across the repository using fast parallel content hashing and calculates total wasted storage space.
- **Full Scan Option (`--full`)**: Bypasses ignore rules to scan all files.
- **Language Breakdown & Metrics**: Computes exact Lines of Code (LOC), blank lines, and comment lines per programming language.
- **Git Health Inspector**: Analyzes total commits, active contributors, top contributors, and last commit dates.
- **Jumbo File Detector**: Flags large files exceeding a customizable threshold (default 5MB).
- **Terminal Visuals & Export**: Formats statistics in clean terminal tables using `comfy-table` and supports exporting reports to `Markdown` (`SCAN_REPORT.md`) or `JSON`.

## Real-World Benchmark

Check out a real-world full benchmark report scanning **34,000+ files and 5.5 Million+ lines of code**:

👉 **[View Full SCAN_REPORT.md](SCAN_REPORT.md)**

## Build from Source

### Build
```bash
cargo build --release
```

### Global Install via Cargo
```bash
cargo install --path .
```

## Usage

### Scan Current Directory (Default)
```bash
ferrum-scan
```

### Storage Breakdown Analysis
```bash
ferrum-scan --storage
# or short flag:
ferrum-scan -s
```

### Duplicate Files Detection & Wasted Space Calculation
```bash
ferrum-scan --duplicates
# or short flag:
ferrum-scan -d
```

### Combine Storage, Duplicate, and Full Analysis
```bash
ferrum-scan --full -s -d
```

### Scan Specific Path
```bash
ferrum-scan /path/to/repository
```

### Export Report to Markdown
```bash
ferrum-scan export --format md --output SCAN_REPORT.md -s -d
```

## License

This project is licensed under the [MIT License.](License)

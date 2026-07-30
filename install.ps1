param(
  [switch]$Uninstall,
  [string]$Version = "latest",
  [string]$RepoOwner = "caya8205-2",
  [string]$RepoName = "ferrum-scan"
)

$AppDir = "$env:LOCALAPPDATA\ferrum-scan"
$BinPath = Join-Path $AppDir 'ferrum-scan.exe'

function Checkmark {
  param([string]$Text)
  Write-Host "  >> $Text" -ForegroundColor Green
}

function AddToPath {
  $current = [Environment]::GetEnvironmentVariable('PATH', 'User')
  if ($current -split ';' -notcontains $AppDir) {
    [Environment]::SetEnvironmentVariable('PATH', "$AppDir;$current", 'User')
    Checkmark "Added to PATH (user-level)"
  } else {
    Write-Host "  ✓ PATH already contains ferrum-scan" -ForegroundColor Cyan
  }
}

function RemoveFromPath {
  $current = [Environment]::GetEnvironmentVariable('PATH', 'User')
  if ($current -and $current.Contains($AppDir)) {
    $new = ($current -split ';' | Where-Object { $_ -ne $AppDir }) -join ';'
    [Environment]::SetEnvironmentVariable('PATH', $new, 'User')
  }
}

function DoUninstall {
  Write-Host "Removing ferrum-scan..." -ForegroundColor Yellow
  if (Test-Path $AppDir) { Remove-Item $AppDir -Recurse -Force; Checkmark "Removed $AppDir" }
  RemoveFromPath
  Write-Host ""
  Checkmark "ferrum-scan uninstalled successfully."
}

# ── Uninstall flow ────────────────────────────────────────
if ($Uninstall) { DoUninstall; return }

# ── Already installed? ────────────────────────────────────
if (Test-Path $BinPath) {
  Write-Host "ferrum-scan is already installed." -ForegroundColor Yellow
  Write-Host "  [U]ninstall   - remove ferrum-scan"
  Write-Host "  [R]einstall   - overwrite binary"
  Write-Host "  [C]ancel      - do nothing"
  $key = (Read-Host "Choice").ToUpper()
  if ($key -eq 'U') { DoUninstall; return }
  if ($key -ne 'R') { Write-Host "Cancelled." -ForegroundColor Gray; return }
}

# ── Install flow ──────────────────────────────────────────
if (-not (Test-Path $AppDir)) { New-Item -ItemType Directory -Path $AppDir -Force | Out-Null }

Write-Host "Fetching release info from GitHub..." -ForegroundColor Cyan

if ($Version -eq "latest") {
  $apiUrl = "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"
} else {
  $apiUrl = "https://api.github.com/repos/$RepoOwner/$RepoName/releases/tags/$Version"
}

try {
  $release = Invoke-RestMethod -Uri $apiUrl -Headers @{ "User-Agent" = "ferrum-scan-installer" }
  $downloadUrl = "https://github.com/$RepoOwner/$RepoName/releases/download/$($release.tag_name)/ferrum-scan.exe"
  
  Write-Host "  Downloading ferrum-scan.exe ($($release.tag_name))..." -NoNewline
  $wc = New-Object System.Net.WebClient
  $wc.DownloadFile($downloadUrl, $BinPath)
  Write-Host " "
  Checkmark "Downloaded ferrum-scan.exe"
} catch {
  Write-Host ""
  Write-Host "ERROR: Failed to fetch release from GitHub." -ForegroundColor Red
  Write-Host "Make sure the repository has published releases." -ForegroundColor Yellow
  Write-Host ""
  Write-Host "You can also download manually from:" -ForegroundColor Cyan
  Write-Host "https://github.com/$RepoOwner/$RepoName/releases" -ForegroundColor White
  exit 1
}

AddToPath
Write-Host ""
Checkmark "Installation complete!"
Write-Host ""
Write-Host "Installed to:" -ForegroundColor Cyan
Write-Host "  $BinPath" -ForegroundColor White
Write-Host ""
Write-Host "Open a new terminal and run: ferrum-scan --help" -ForegroundColor Yellow

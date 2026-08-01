#!/usr/bin/env bash
set -e

REPO_OWNER="caya8205-2"
REPO_NAME="ferrum-scan"
INSTALL_DIR="$HOME/.ferrum-scan/bin"
BIN_PATH="$INSTALL_DIR/ferrum-scan"

if [ "$1" = "--uninstall" ] || [ "$1" = "-u" ]; then
  echo "Removing ferrum-scan..."
  if [ -d "$HOME/.ferrum-scan" ]; then
    rm -rf "$HOME/.ferrum-scan"
    echo "  ✓ Removed $HOME/.ferrum-scan"
  fi
  
  # Hapus baris PATH dari .bashrc jika ada
  if [ -f "$HOME/.bashrc" ]; then
    sed -i '\/\.ferrum-scan\/bin/d' "$HOME/.bashrc"
    echo "  ✓ Removed PATH from .bashrc"
  fi
  # Hapus baris PATH dari .zshrc jika ada
  if [ -f "$HOME/.zshrc" ]; then
    sed -i '\/\.ferrum-scan\/bin/d' "$HOME/.zshrc"
    echo "  ✓ Removed PATH from .zshrc"
  fi
  if [ -L "$HOME/.local/bin/ferrum-scan" ] || [ -f "$HOME/.local/bin/ferrum-scan" ]; then
    rm -f "$HOME/.local/bin/ferrum-scan"
    echo "  ✓ Removed $HOME/.local/bin/ferrum-scan"
  fi

  echo ""
  echo "✅ ferrum-scan uninstalled successfully."
  exit 0
fi

echo "Fetching release info from GitHub..."
RELEASE_JSON=$(curl -s "https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest")
TAG_NAME=$(echo "$RELEASE_JSON" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$OS" = "linux" ]; then
  BINARY_NAME="ferrum-scan-linux"
elif [ "$OS" = "darwin" ]; then
  BINARY_NAME="ferrum-scan-macos"
else
  echo "Unsupported OS: $OS"
  exit 1
fi

DOWNLOAD_URL="https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$TAG_NAME/$BINARY_NAME"

mkdir -p "$INSTALL_DIR"
echo "Downloading $BINARY_NAME ($TAG_NAME)..."
curl -fsSL "$DOWNLOAD_URL" -o "$BIN_PATH"
chmod +x "$BIN_PATH"

echo "✅ Installation complete!"
echo "Installed to: $BIN_PATH"
echo ""

# --- BAGIAN OTOMATISASI PATH ---
PATH_LINE='export PATH="$HOME/.ferrum-scan/bin:$PATH"'

# 1. Symlink ke ~/.local/bin (biasanya sudah ada di default PATH Linux)
if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]] || [ -d "$HOME/.local/bin" ] || [ -d "/usr/local/bin" ]; then
  LOCAL_BIN="$HOME/.local/bin"
  mkdir -p "$LOCAL_BIN"
  ln -sf "$BIN_PATH" "$LOCAL_BIN/ferrum-scan"
  echo "✓ Created symlink in $LOCAL_BIN/ferrum-scan"
fi

# 2. Tambahkan ke file RC shell (.bashrc / .zshrc) agar permanen di shell baru
TARGET_RCS=()
[ -f "$HOME/.bashrc" ] && TARGET_RCS+=("$HOME/.bashrc")
[ -f "$HOME/.zshrc" ] && TARGET_RCS+=("$HOME/.zshrc")

for RC in "${TARGET_RCS[@]}"; do
  if ! grep -q ".ferrum-scan/bin" "$RC"; then
    echo "" >> "$RC"
    echo "$PATH_LINE" >> "$RC"
    echo "✓ Added to PATH in $RC"
  fi
done

# 3. Cek apakah ferrum-scan langsung dapat dipanggil di terminal saat ini
if command -v ferrum-scan >/dev/null 2>&1 || [[ ":$PATH:" == *":$INSTALL_DIR:"* ]] || [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
  echo ""
  echo "🎉 ferrum-scan is ready to use!"
  echo "👉 Try running: ferrum-scan --help"
else
  echo ""
  if [ -n "${TARGET_RCS[0]}" ]; then
    echo "👉 Run: source ${TARGET_RCS[0]} (or restart your terminal) to use 'ferrum-scan'"
  fi
fi

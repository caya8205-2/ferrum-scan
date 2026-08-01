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

  echo ""
  echo "✅ ferrum-scan uninstalled successfully."
  exit 0
fi

echo "Fetching release info from GitHub..."
RELEASE_JSON=$(curl -s "https://github.com")
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

DOWNLOAD_URL="https://github.com"

mkdir -p "$INSTALL_DIR"
echo "Downloading $BINARY_NAME ($TAG_NAME)..."
curl -fsSL "$DOWNLOAD_URL" -o "$BIN_PATH"
chmod +x "$BIN_PATH"

echo "✅ Installation complete!"
echo "Installed to: $BIN_PATH"
echo ""

# --- BAGIAN OTOMATISASI PATH ---
PATH_LINE='export PATH="$HOME/.ferrum-scan/bin:$PATH"'
SHELL_RC=""

if [ -n "$ZSH_VERSION" ] || [ -f "$HOME/.zshrc" ]; then
  SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ] || [ -f "$HOME/.bashrc" ]; then
  SHELL_RC="$HOME/.bashrc"
fi

if [ -n "$SHELL_RC" ]; then
  if ! grep -q ".ferrum-scan/bin" "$SHELL_RC"; then
    echo "" >> "$SHELL_RC"
    echo "$PATH_LINE" >> "$SHELL_RC"
    echo "✓ Added to PATH in $SHELL_RC"
  else
    echo "✓ PATH already exists in $SHELL_RC"
  fi
  echo "👉 Please run: source $SHELL_RC (or restart your terminal) to use 'ferrum-scan'"
else
  echo "Add to PATH manually: $PATH_LINE"
fi

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
echo "Add to PATH: export PATH=\"\$HOME/.ferrum-scan/bin:\$PATH\""

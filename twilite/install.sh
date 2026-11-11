#!/usr/bin/env bash

set -e

echo "Building Twilite..."

cargo build --release

echo "Installing executable..."

mkdir -p ~/.local/bin
install -Dm755 target/release/twilite ~/.local/bin/twilite

echo "Installing desktop entry..."

mkdir -p ~/.local/share/applications
install -Dm644 assets/twilite.desktop ~/.local/share/applications/twilite.desktop

echo "Installing icon..."

mkdir -p ~/.local/share/icons/hicolor/scalable/apps
install -Dm644 assets/twilite.svg ~/.local/share/icons/hicolor/scalable/apps/twilite.svg

echo "Creating configuration..."

mkdir -p ~/.config/twilite

if [ ! -f ~/.config/twilite/config.toml ]; then
cat > ~/.config/twilite/config.toml << EOF
# Paste the paths to your actual wallpapers.

light = "/path/to/light.png"
dark = "/path/to/dark.png"
EOF
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database ~/.local/share/applications >/dev/null 2>&1 || true
fi

echo
echo "Twilite has been installed successfully!"
echo
echo "Please modify the config file before launching Twilite:"
echo
echo "~/.config/twilite/config.toml"
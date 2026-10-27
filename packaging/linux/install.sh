#!/bin/sh
set -eu

SOURCE_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP_DIR="${HOME}/.local/opt/qrcode-maker"
DATA_DIR="${XDG_DATA_HOME:-${HOME}/.local/share}"
APPLICATIONS_DIR="${DATA_DIR}/applications"
ICONS_DIR="${DATA_DIR}/icons/hicolor"

mkdir -p "$APP_DIR" "$APPLICATIONS_DIR"
install -m 755 "$SOURCE_DIR/QRCodeMaker" "$APP_DIR/QRCodeMaker"

# 圖示：Icon=qrcode-maker 會對應到這兩個檔案
install -Dm644 "$SOURCE_DIR/app-icon.svg" "$ICONS_DIR/scalable/apps/qrcode-maker.svg"
install -Dm644 "$SOURCE_DIR/app-icon.png" "$ICONS_DIR/512x512/apps/qrcode-maker.png"

cat >"$APPLICATIONS_DIR/qrcode-maker.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=QR Code Maker
Comment=Create QR codes for text, URLs, and WiFi
Exec="$APP_DIR/QRCodeMaker"
Icon=qrcode-maker
Terminal=false
Categories=Utility;
StartupWMClass=qrcode-maker
EOF

# 更新快取；失敗不影響安裝
gtk-update-icon-cache -f -t "$ICONS_DIR" >/dev/null 2>&1 || true
update-desktop-database "$APPLICATIONS_DIR" >/dev/null 2>&1 || true

printf 'Installed QR Code Maker. It should now appear in your desktop app menu.\n'

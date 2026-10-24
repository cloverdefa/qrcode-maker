#!/bin/sh
set -eu

SOURCE_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP_DIR="${HOME}/.local/opt/qrcode-maker"
APPLICATIONS_DIR="${XDG_DATA_HOME:-${HOME}/.local/share}/applications"

mkdir -p "$APP_DIR" "$APPLICATIONS_DIR"
install -m 755 "$SOURCE_DIR/QRCodeMaker" "$APP_DIR/QRCodeMaker"
cat > "$APPLICATIONS_DIR/qrcode-maker.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=QR Code Maker
Comment=Create QR codes for text, URLs, and WiFi
Exec="$APP_DIR/QRCodeMaker"
Terminal=false
Categories=Utility;
EOF

printf 'Installed QR Code Maker. It should now appear in your desktop app menu.\n'

#!/bin/sh
set -eu

APP_DIR="${HOME}/.local/opt/qrcode-maker"
APPLICATIONS_DIR="${XDG_DATA_HOME:-${HOME}/.local/share}/applications"
rm -f "$APP_DIR/QRCodeMaker" "$APPLICATIONS_DIR/qrcode-maker.desktop"
rmdir "$APP_DIR" 2>/dev/null || true
rmdir "$(dirname -- "$APP_DIR")" 2>/dev/null || true
printf 'Removed QR Code Maker.\n'

#!/bin/sh
set -eu

APP_DIR="${HOME}/.local/opt/qrcode-maker"
DATA_DIR="${XDG_DATA_HOME:-${HOME}/.local/share}"
APPLICATIONS_DIR="${DATA_DIR}/applications"
ICONS_DIR="${DATA_DIR}/icons/hicolor"

rm -rf "$APP_DIR"
rm -f "$APPLICATIONS_DIR/qrcode-maker.desktop" \
      "$ICONS_DIR/scalable/apps/qrcode-maker.svg" \
      "$ICONS_DIR/512x512/apps/qrcode-maker.png"

gtk-update-icon-cache -f -t "$ICONS_DIR" >/dev/null 2>&1 || true
update-desktop-database "$APPLICATIONS_DIR" >/dev/null 2>&1 || true

printf 'Uninstalled QR Code Maker.\n'

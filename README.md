# QR Code Maker

以 Rust 和 egui 製作的桌面 QR code 產生器。可將文字、網址或 WiFi 資訊轉成 QR code、預覽結果，並儲存為 PNG 或 JPEG 圖片。

## 功能

- 將一般文字或網址轉成 QR code。
- 產生 WiFi QR code，支援 WPA、WEP、無密碼與隱藏網路。
- 內嵌繁體中文字型，Windows 與 Linux 執行檔皆可正常顯示中文。
- 選擇錯誤修正等級：低（L）、中（M）、較高（Q）或高（H）。
- 設定模組大小（1–40）；預設為 10。
- 預覽 QR code，並以 PNG 或 JPEG 格式另存。
- 按 `Ctrl + Enter` 快速產生 QR code。

## 使用示範

![QR Code Maker 最新介面示範：精簡初始視窗、輸入文字、產生 QR code 後展開預覽，並使用另存或清除操作](assets/qrcode-maker-demo.gif)

## 環境需求

- Rust stable（含 Cargo）。
- 可用的桌面圖形環境。

## 執行

在專案目錄執行：

```bash
cargo run --release
```

## 建置 Windows 執行檔

在 Windows 環境執行：

```powershell
cargo build --release
```

執行檔位於 `target\release\QRCodeMaker.exe`。請在目標作業系統上建置應用程式。

## Linux x86_64 桌面版（Omarchy / WSLg）

GitHub Release 也會提供 `qrcode-maker-linux-x86_64.tar.gz`。解壓後執行 `./install.sh`，即可安裝到使用者目錄並加入桌面應用程式選單；也可以直接執行 `./QRCodeMaker`。移除時執行 `./uninstall.sh`。

此版本適用於 Omarchy 等 64 位元 Linux 桌面，以及啟用 WSLg 的 WSL2。主機需要 X11 或 Wayland 圖形環境；儲存對話框使用 XDG Desktop Portal。

## 專案檔案

- `src/main.rs`：egui 圖形介面、QR code 產生及圖片儲存。
- `Cargo.toml`：Rust 專案資訊與依賴設定。
- `packaging/linux/`：Linux 桌面版安裝、移除指令與說明。
- `assets/fonts/`：內嵌的 Noto Sans TC 字型與 SIL Open Font License 授權。

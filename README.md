# QR Code Maker

以 Rust 和 egui 製作的桌面 QR code 產生器。可將文字、網址或 WiFi 資訊轉成 QR code、預覽結果，並儲存為 PNG 或 JPEG 圖片。

## 功能

- 將一般文字或網址轉成 QR code。
- 產生 WiFi QR code，支援 WPA、WEP、無密碼與隱藏網路。
- 選擇錯誤修正等級：低（L）、中（M）、較高（Q）或高（H）。
- 設定模組大小（1–40）；預設為 10。
- 預覽 QR code，並以 PNG 或 JPEG 格式另存。
- 按 `Ctrl + Enter` 快速產生 QR code。

## 使用示範

![QR Code Maker 示範：輸入網址、產生 QR code 並儲存](assets/qrcode-maker-demo.gif)

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

## 專案檔案

- `src/main.rs`：egui 圖形介面、QR code 產生及圖片儲存。
- `Cargo.toml`：Rust 專案資訊與依賴設定。

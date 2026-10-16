# QR Code Maker

以 Python 和 Tkinter 製作的桌面 QR code 產生器。輸入文字或網址即可產生 QR code、預覽結果，並儲存為 PNG 或 JPEG 圖片。

## 功能

- 將一般文字或網址轉成 QR code。
- 選擇錯誤修正等級：低（L）、中（M）、較高（Q）或高（H）。
- 設定模組大小（1–40）；預設為 10。
- 預覽產生的 QR code，並以 PNG 或 JPEG 格式另存。
- 按 `Ctrl + Enter` 快速產生 QR code。

## 使用示範

![QR Code Maker 示範：輸入網址、產生 QR code 並儲存](assets/qrcode-maker-demo.gif)

## 環境需求

- Python 3.10 或更新版本。
- [uv](https://docs.astral.sh/uv/) 套件管理工具。
- 可用的 Tkinter 桌面環境。部分 Linux 發行版需另外安裝 Tkinter 系統套件。

## 安裝與執行

在專案目錄執行：

```bash
uv sync
uv run python qrcode_maker.py
```

也可以透過已設定的命令列入口啟動：

```bash
uv run qrcode-maker
```

## 建置 Windows 執行檔

在 Windows 環境的專案目錄執行：

```powershell
uv run pyinstaller --noconfirm --clean QRCodeMaker.spec
```

建置完成後，執行檔會位於 `dist\QRCodeMaker.exe`。PyInstaller 應用程式建議在目標作業系統上建置，因此 Windows 執行檔請在 Windows 上產生。

## 專案檔案

- `qrcode_maker.py`：Tkinter 圖形介面與 QR code 產生、儲存功能。
- `QRCodeMaker.spec`：PyInstaller Windows GUI 執行檔建置設定。
- `pyproject.toml`：專案資訊、執行依賴及 `qrcode-maker` 命令列入口。

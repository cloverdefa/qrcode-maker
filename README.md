# QR Code Maker

Windows 桌面 QR code 產生器。輸入一般文字或 URL，即可產生 QR code 預覽並另存為 PNG 或 JPEG。介面使用 Python 內建的 Tkinter 建立。

## 使用示範

![QR Code Maker 使用示範：輸入網址、產生 QR code 並儲存](assets/qrcode-maker-demo.gif)

## 建立環境

先安裝 [uv](https://docs.astral.sh/uv/)，然後在專案資料夾執行：

```powershell
uv sync
```

這會建立 `.venv` 並安裝應用程式及開發依賴。

## 執行

```powershell
uv run python qrcode_maker.py
```

也可以安裝成命令列入口後執行：

```powershell
uv run qrcode-maker
```

## 建立 Windows 應用程式

在 Windows 上執行以下命令，產生不顯示終端視窗的單一執行檔：

```powershell
uv run pyinstaller --noconfirm --clean QRCodeMaker.spec
```

完成後，執行檔位於 `dist\QRCodeMaker.exe`。PyInstaller 建議在目標作業系統上建置，因此 Windows `.exe` 請於 Windows 環境建立。

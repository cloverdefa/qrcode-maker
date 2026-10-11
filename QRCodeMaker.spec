# PyInstaller spec for the QR Code Maker Windows GUI application.
# Build from the project root with: uv run pyinstaller --noconfirm --clean QRCodeMaker.spec

from PyInstaller.utils.hooks import collect_all

qrcode_datas, qrcode_binaries, qrcode_hiddenimports = collect_all("qrcode")
pil_datas, pil_binaries, pil_hiddenimports = collect_all("PIL")

a = Analysis(
    ["qrcode_maker.py"],
    pathex=[],
    binaries=qrcode_binaries + pil_binaries,
    datas=qrcode_datas + pil_datas,
    hiddenimports=qrcode_hiddenimports + pil_hiddenimports,
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
    optimize=0,
)
pyz = PYZ(a.pure)
exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name="QRCodeMaker",
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=False,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)

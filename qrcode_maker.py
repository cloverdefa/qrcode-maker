"""A small desktop QR code maker built with Tkinter."""

from __future__ import annotations

import tkinter as tk
from tkinter import filedialog, messagebox, ttk

import qrcode
from PIL import Image, ImageTk
from qrcode.constants import ERROR_CORRECT_H, ERROR_CORRECT_L, ERROR_CORRECT_M, ERROR_CORRECT_Q


ERROR_LEVELS = {
    "低 (L)": ERROR_CORRECT_L,
    "中 (M)": ERROR_CORRECT_M,
    "較高 (Q)": ERROR_CORRECT_Q,
    "高 (H)": ERROR_CORRECT_H,
}


class QRCodeMaker(tk.Tk):
    def __init__(self) -> None:
        super().__init__()
        self.title("QR Code Maker")
        self.geometry("560x690")
        self.minsize(480, 620)
        self.configure(bg="#f4f6f8")

        self.qr_image: Image.Image | None = None
        self.preview_image: ImageTk.PhotoImage | None = None
        self.error_level = tk.StringVar(value="中 (M)")
        self.box_size = tk.IntVar(value=10)
        self._build_ui()

    def _build_ui(self) -> None:
        style = ttk.Style(self)
        style.theme_use("clam")
        style.configure("TFrame", background="#f4f6f8")
        style.configure("TLabel", background="#f4f6f8", foreground="#17212b", font=("Segoe UI", 10))
        style.configure("Title.TLabel", font=("Segoe UI", 22, "bold"), foreground="#102a43")
        style.configure("Hint.TLabel", foreground="#52606d")
        style.configure("TButton", font=("Segoe UI", 10), padding=(12, 8))
        style.configure("Accent.TButton", font=("Segoe UI", 10, "bold"), padding=(12, 9))

        root = ttk.Frame(self, padding=24)
        root.pack(fill="both", expand=True)
        ttk.Label(root, text="QR Code Maker", style="Title.TLabel").pack(anchor="w")
        ttk.Label(root, text="輸入文字或網址，立即產生 QR code。", style="Hint.TLabel").pack(anchor="w", pady=(4, 16))

        ttk.Label(root, text="內容").pack(anchor="w")
        self.input = tk.Text(root, height=4, wrap="word", font=("Segoe UI", 11), padx=10, pady=8,
                             relief="solid", bd=1, highlightthickness=1,
                             highlightbackground="#cbd2d9", highlightcolor="#2680c2")
        self.input.pack(fill="x", pady=(6, 14))
        self.input.bind("<Control-Return>", lambda _event: self.generate())

        options = ttk.Frame(root)
        options.pack(fill="x", pady=(0, 14))
        ttk.Label(options, text="錯誤修正").grid(row=0, column=0, sticky="w")
        ttk.Combobox(options, textvariable=self.error_level, values=list(ERROR_LEVELS),
                     state="readonly", width=12).grid(row=1, column=0, sticky="w", pady=(4, 0))
        ttk.Label(options, text="模組大小").grid(row=0, column=1, sticky="w", padx=(18, 0))
        ttk.Spinbox(options, from_=4, to=20, textvariable=self.box_size, width=8).grid(
            row=1, column=1, sticky="w", padx=(18, 0), pady=(4, 0))
        options.columnconfigure(2, weight=1)

        actions = ttk.Frame(root)
        actions.pack(fill="x")
        ttk.Button(actions, text="產生 QR Code", style="Accent.TButton", command=self.generate).pack(side="left")
        self.save_button = ttk.Button(actions, text="另存 PNG", command=self.save, state="disabled")
        self.save_button.pack(side="left", padx=(8, 0))

        self.preview = tk.Label(root, text="QR code 預覽會顯示在這裡", bg="white", fg="#829ab1",
                                font=("Segoe UI", 10), relief="solid", bd=1)
        self.preview.pack(fill="both", expand=True, pady=(18, 8))
        self.status = ttk.Label(root, text="提示：按 Ctrl + Enter 可快速產生。", style="Hint.TLabel")
        self.status.pack(anchor="w")

    def generate(self) -> None:
        content = self.input.get("1.0", "end-1c").strip()
        if not content:
            messagebox.showinfo("需要內容", "請先輸入文字或網址。")
            self.input.focus_set()
            return
        try:
            size = int(self.box_size.get())
            if not 1 <= size <= 40:
                raise ValueError
            qr = qrcode.QRCode(version=None, error_correction=ERROR_LEVELS[self.error_level.get()],
                               box_size=size, border=4)
            qr.add_data(content)
            qr.make(fit=True)
            self.qr_image = qr.make_image(fill_color="black", back_color="white").convert("RGB")
            display = self.qr_image.copy()
            display.thumbnail((360, 360), Image.Resampling.NEAREST)
            self.preview_image = ImageTk.PhotoImage(display)
            self.preview.configure(image=self.preview_image, text="", padx=0, pady=0)
            self.save_button.configure(state="normal")
            self.status.configure(text=f"已產生 {qr.modules_count} × {qr.modules_count} QR code。")
        except (ValueError, KeyError):
            messagebox.showerror("設定錯誤", "模組大小請輸入 1 到 40 之間的整數。")
        except Exception as exc:
            messagebox.showerror("產生失敗", f"無法產生 QR code：{exc}")

    def save(self) -> None:
        if self.qr_image is None:
            return
        path = filedialog.asksaveasfilename(
            title="儲存 QR code", defaultextension=".png",
            filetypes=[("PNG 圖片", "*.png"), ("JPEG 圖片", "*.jpg;*.jpeg"), ("所有檔案", "*.*")],
            initialfile="qrcode.png",
        )
        if path:
            self.qr_image.save(path)
            self.status.configure(text=f"已儲存：{path}")


def main() -> None:
    QRCodeMaker().mainloop()


if __name__ == "__main__":
    main()

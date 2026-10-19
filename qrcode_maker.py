"""A small desktop QR code maker built with Tkinter (text / URL / WiFi)."""

from __future__ import annotations

import re
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

# 顯示名稱 -> WiFi QR code 的 T 欄位
WIFI_SECURITY = {
    "WPA / WPA2 / WPA3": "WPA",
    "WEP": "WEP",
    "無密碼": "nopass",
}

MODE_TEXT = "text"
MODE_WIFI = "wifi"


def _wifi_escape(value: str) -> str:
    """跳脫 WiFi QR 格式中的特殊字元：\\ ; , : \""""
    for ch in ('\\', ';', ',', ':', '"'):
        value = value.replace(ch, "\\" + ch)
    # 若全是十六進位字元，部分裝置會誤判為 hex，需加雙引號
    if value and re.fullmatch(r"[0-9A-Fa-f]+", value):
        value = f'"{value}"'
    return value


def build_wifi_payload(ssid: str, password: str, security: str, hidden: bool) -> str:
    """產生 WIFI:T:WPA;S:ssid;P:password;H:true;; 格式字串。"""
    parts = [f"WIFI:T:{security}", f"S:{_wifi_escape(ssid)}"]
    if security != "nopass":
        parts.append(f"P:{_wifi_escape(password)}")
    if hidden:
        parts.append("H:true")
    return ";".join(parts) + ";;"


class QRCodeMaker(tk.Tk):
    def __init__(self) -> None:
        super().__init__()
        self.title("QR Code Maker")
        self.geometry("560x780")
        self.minsize(480, 700)
        self.configure(bg="#f4f6f8")

        self.qr_image: Image.Image | None = None
        self.preview_image: ImageTk.PhotoImage | None = None
        self.mode = tk.StringVar(value=MODE_TEXT)
        self.error_level = tk.StringVar(value="中 (M)")
        self.box_size = tk.IntVar(value=10)
        self.wifi_ssid = tk.StringVar()
        self.wifi_password = tk.StringVar()
        self.wifi_security = tk.StringVar(value="WPA / WPA2 / WPA3")
        self.wifi_hidden = tk.BooleanVar(value=False)
        self.show_password = tk.BooleanVar(value=False)
        self._build_ui()
        self._on_mode_change()

    def _build_ui(self) -> None:
        style = ttk.Style(self)
        style.theme_use("clam")
        style.configure("TFrame", background="#f4f6f8")
        style.configure("TLabel", background="#f4f6f8", foreground="#17212b", font=("Segoe UI", 10))
        style.configure("Title.TLabel", font=("Segoe UI", 22, "bold"), foreground="#102a43")
        style.configure("Hint.TLabel", foreground="#52606d")
        style.configure("TButton", font=("Segoe UI", 10), padding=(12, 8))
        style.configure("Accent.TButton", font=("Segoe UI", 10, "bold"), padding=(12, 9))
        style.configure("TRadiobutton", background="#f4f6f8", font=("Segoe UI", 10))
        style.configure("TCheckbutton", background="#f4f6f8", font=("Segoe UI", 10))

        root = ttk.Frame(self, padding=24)
        root.pack(fill="both", expand=True)
        ttk.Label(root, text="QR Code Maker", style="Title.TLabel").pack(anchor="w")
        ttk.Label(root, text="輸入文字、網址或 WiFi 資訊，立即產生 QR code。",
                  style="Hint.TLabel").pack(anchor="w", pady=(4, 12))

        # --- 模式切換 ---
        mode_bar = ttk.Frame(root)
        mode_bar.pack(fill="x", pady=(0, 12))
        ttk.Radiobutton(mode_bar, text="文字 / 網址", value=MODE_TEXT, variable=self.mode,
                        command=self._on_mode_change).pack(side="left")
        ttk.Radiobutton(mode_bar, text="WiFi", value=MODE_WIFI, variable=self.mode,
                        command=self._on_mode_change).pack(side="left", padx=(16, 0))

        # --- 內容區（依模式切換） ---
        self.content_holder = ttk.Frame(root)
        self.content_holder.pack(fill="x")

        # 文字 / 網址
        self.text_frame = ttk.Frame(self.content_holder)
        ttk.Label(self.text_frame, text="內容").pack(anchor="w")
        self.input = tk.Text(self.text_frame, height=4, wrap="word", font=("Segoe UI", 11), padx=10, pady=8,
                             relief="solid", bd=1, highlightthickness=1,
                             highlightbackground="#cbd2d9", highlightcolor="#2680c2")
        self.input.pack(fill="x", pady=(6, 14))
        self.input.bind("<Control-Return>", lambda _event: self.generate())

        # WiFi
        self.wifi_frame = ttk.Frame(self.content_holder)
        self.wifi_frame.columnconfigure(0, weight=1)
        ttk.Label(self.wifi_frame, text="WiFi 名稱 (SSID)").grid(row=0, column=0, sticky="w")
        ssid_entry = ttk.Entry(self.wifi_frame, textvariable=self.wifi_ssid, font=("Segoe UI", 11))
        ssid_entry.grid(row=1, column=0, sticky="ew", pady=(6, 10))
        ttk.Label(self.wifi_frame, text="密碼").grid(row=2, column=0, sticky="w")
        self.password_entry = ttk.Entry(self.wifi_frame, textvariable=self.wifi_password,
                                        show="•", font=("Segoe UI", 11))
        self.password_entry.grid(row=3, column=0, sticky="ew", pady=(6, 4))
        ttk.Checkbutton(self.wifi_frame, text="顯示密碼", variable=self.show_password,
                        command=self._toggle_password).grid(row=4, column=0, sticky="w", pady=(0, 10))
        ttk.Label(self.wifi_frame, text="加密方式").grid(row=5, column=0, sticky="w")
        security_box = ttk.Combobox(self.wifi_frame, textvariable=self.wifi_security,
                                    values=list(WIFI_SECURITY), state="readonly", width=20)
        security_box.grid(row=6, column=0, sticky="w", pady=(6, 10))
        security_box.bind("<<ComboboxSelected>>", lambda _e: self._update_password_state())
        ttk.Checkbutton(self.wifi_frame, text="隱藏的網路 (Hidden SSID)",
                        variable=self.wifi_hidden).grid(row=7, column=0, sticky="w", pady=(0, 14))
        for widget in (ssid_entry, self.password_entry):
            widget.bind("<Return>", lambda _event: self.generate())

        # --- 選項 ---
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

    # ---------- 模式 / 欄位狀態 ----------
    def _on_mode_change(self) -> None:
        self.text_frame.pack_forget()
        self.wifi_frame.pack_forget()
        if self.mode.get() == MODE_WIFI:
            self.wifi_frame.pack(fill="x")
            self._update_password_state()
        else:
            self.text_frame.pack(fill="x")

    def _toggle_password(self) -> None:
        self.password_entry.configure(show="" if self.show_password.get() else "•")

    def _update_password_state(self) -> None:
        nopass = WIFI_SECURITY[self.wifi_security.get()] == "nopass"
        self.password_entry.configure(state="disabled" if nopass else "normal")

    # ---------- 產生內容 ----------
    def _get_payload(self) -> str | None:
        """依目前模式取得要編碼的字串；驗證失敗時顯示訊息並回傳 None。"""
        if self.mode.get() == MODE_WIFI:
            ssid = self.wifi_ssid.get()
            security = WIFI_SECURITY[self.wifi_security.get()]
            password = self.wifi_password.get()
            if not ssid.strip():
                messagebox.showinfo("需要內容", "請輸入 WiFi 名稱 (SSID)。")
                return None
            if security != "nopass" and not password:
                messagebox.showinfo("需要內容", "請輸入 WiFi 密碼，或將加密方式改為「無密碼」。")
                self.password_entry.focus_set()
                return None
            return build_wifi_payload(ssid, password, security, self.wifi_hidden.get())

        content = self.input.get("1.0", "end-1c").strip()
        if not content:
            messagebox.showinfo("需要內容", "請先輸入文字或網址。")
            self.input.focus_set()
            return None
        return content

    def generate(self) -> None:
        content = self._get_payload()
        if content is None:
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
            kind = "WiFi " if self.mode.get() == MODE_WIFI else ""
            self.status.configure(text=f"已產生 {kind}{qr.modules_count} × {qr.modules_count} QR code。")
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

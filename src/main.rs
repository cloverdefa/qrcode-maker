use eframe::egui;
use image::{DynamicImage, GrayImage, ImageFormat, Luma};
use qrcode::{EcLevel, QrCode};
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Text,
    Wifi,
}

struct QrCodeMaker {
    mode: Mode,
    text: String,
    ssid: String,
    password: String,
    security: usize,
    hidden: bool,
    show_password: bool,
    error_level: usize,
    box_size: u32,
    qr: Option<GrayImage>,
    texture: Option<egui::TextureHandle>,
    status: String,
}

impl Default for QrCodeMaker {
    fn default() -> Self {
        Self {
            mode: Mode::Text,
            text: String::new(),
            ssid: String::new(),
            password: String::new(),
            security: 0,
            hidden: false,
            show_password: false,
            error_level: 1,
            box_size: 10,
            qr: None,
            texture: None,
            status: "提示：按 Ctrl + Enter 可快速產生。".into(),
        }
    }
}

impl QrCodeMaker {
    fn payload(&self) -> Result<String, String> {
        if self.mode == Mode::Text {
            let text = self.text.trim();
            return if text.is_empty() {
                Err("請先輸入文字或網址。".into())
            } else {
                Ok(text.to_owned())
            };
        }
        if self.ssid.trim().is_empty() {
            return Err("請輸入 WiFi 名稱 (SSID)。".into());
        }
        let security = ["WPA", "WEP", "nopass"][self.security];
        if security != "nopass" && self.password.is_empty() {
            return Err("請輸入 WiFi 密碼，或將加密方式改為「無密碼」。".into());
        }
        let mut parts = vec![format!("WIFI:T:{security}"), format!("S:{}", wifi_escape(&self.ssid))];
        if security != "nopass" {
            parts.push(format!("P:{}", wifi_escape(&self.password)));
        }
        if self.hidden {
            parts.push("H:true".into());
        }
        Ok(format!("{};;", parts.join(";")))
    }

    fn generate(&mut self, ctx: &egui::Context) {
        let payload = match self.payload() {
            Ok(payload) => payload,
            Err(message) => {
                self.status = message;
                return;
            }
        };
        let levels = [EcLevel::L, EcLevel::M, EcLevel::Q, EcLevel::H];
        match QrCode::with_error_correction_level(payload.as_bytes(), levels[self.error_level]) {
            Ok(code) => {
                let side = code.width() as u32;
                let qr = code.render::<Luma<u8>>().quiet_zone(true).module_dimensions(self.box_size, self.box_size).build();
                let rgba: Vec<u8> = qr.pixels().flat_map(|p| [p[0], p[0], p[0], 255]).collect();
                let image = egui::ColorImage::from_rgba_unmultiplied([qr.width() as usize, qr.height() as usize], &rgba);
                self.texture = Some(ctx.load_texture("qr-preview", image, egui::TextureOptions::NEAREST));
                self.qr = Some(qr);
                self.status = format!("已產生{}{} × {} QR code。", if self.mode == Mode::Wifi { "WiFi " } else { "" }, side, side);
            }
            Err(error) => self.status = format!("無法產生 QR code：{error}"),
        }
    }

    fn save(&mut self) {
        let Some(qr) = &self.qr else { return };
        let Some(path) = rfd::FileDialog::new()
            .set_title("儲存 QR code")
            .set_file_name("qrcode.png")
            .add_filter("PNG 圖片", &["png"])
            .add_filter("JPEG 圖片", &["jpg", "jpeg"])
            .save_file() else { return };
        let format = match Path::new(&path).extension().and_then(|ext| ext.to_str()).map(str::to_ascii_lowercase).as_deref() {
            Some("jpg" | "jpeg") => ImageFormat::Jpeg,
            _ => ImageFormat::Png,
        };
        let image = DynamicImage::ImageLuma8(qr.clone());
        match image.save_with_format(&path, format) {
            Ok(()) => self.status = format!("已儲存：{}", path.display()),
            Err(error) => self.status = format!("儲存失敗：{error}"),
        }
    }
}

fn wifi_escape(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        if ['\\', ';', ',', ':', '"'].contains(&ch) { escaped.push('\\'); }
        escaped.push(ch);
    }
    if !escaped.is_empty() && escaped.chars().all(|ch| ch.is_ascii_hexdigit()) {
        format!("\"{escaped}\"")
    } else { escaped }
}

impl eframe::App for QrCodeMaker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter)) { self.generate(ctx); }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("QR Code Maker");
            ui.label("輸入文字、網址或 WiFi 資訊，立即產生 QR code。");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, Mode::Text, "文字 / 網址");
                ui.selectable_value(&mut self.mode, Mode::Wifi, "WiFi");
            });
            ui.add_space(8.0);
            if self.mode == Mode::Text {
                ui.label("內容");
                ui.add(egui::TextEdit::multiline(&mut self.text).desired_rows(4).hint_text("輸入文字或網址"));
            } else {
                ui.label("WiFi 名稱 (SSID)");
                ui.text_edit_singleline(&mut self.ssid);
                ui.label("密碼");
                ui.add(egui::TextEdit::singleline(&mut self.password).password(!self.show_password).interactive(self.security != 2));
                ui.checkbox(&mut self.show_password, "顯示密碼");
                ui.label("加密方式");
                egui::ComboBox::from_id_salt("security").selected_text(["WPA / WPA2 / WPA3", "WEP", "無密碼"][self.security]).show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.security, 0, "WPA / WPA2 / WPA3");
                    ui.selectable_value(&mut self.security, 1, "WEP");
                    ui.selectable_value(&mut self.security, 2, "無密碼");
                });
                ui.checkbox(&mut self.hidden, "隱藏的網路 (Hidden SSID)");
            }
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("錯誤修正");
                egui::ComboBox::from_id_salt("error-level").selected_text(["低 (L)", "中 (M)", "較高 (Q)", "高 (H)"][self.error_level]).show_ui(ui, |ui| {
                    for (index, name) in ["低 (L)", "中 (M)", "較高 (Q)", "高 (H)"].iter().enumerate() { ui.selectable_value(&mut self.error_level, index, *name); }
                });
                ui.label("模組大小");
                ui.add(egui::DragValue::new(&mut self.box_size).range(1..=40));
            });
            ui.horizontal(|ui| {
                if ui.button("產生 QR Code").clicked() { self.generate(ctx); }
                if ui.add_enabled(self.qr.is_some(), egui::Button::new("另存 PNG / JPEG")).clicked() { self.save(); }
            });
            ui.add_space(10.0);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_height(300.0);
                ui.set_width(ui.available_width());
                if let Some(texture) = &self.texture {
                    let available = ui.available_size();
                    let size = texture.size_vec2().x.min(texture.size_vec2().y).min(available.x).min(available.y);
                    ui.centered_and_justified(|ui| { ui.image((texture.id(), egui::vec2(size, size))); });
                } else { ui.centered_and_justified(|ui| { ui.label("QR code 預覽會顯示在這裡"); }); }
            });
            ui.label(&self.status);
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([560.0, 780.0]).with_min_inner_size([480.0, 700.0]), ..Default::default() };
    eframe::run_native("QR Code Maker", options, Box::new(|_| Ok(Box::<QrCodeMaker>::default())))
}

//! 應用程式狀態與核心邏輯。

use crate::payload::Form;
use crate::update::{self, UpdateState};
use crate::{qr, theme};
use eframe::egui;
use image::GrayImage;
use std::sync::mpsc::Receiver;

pub const INITIAL_SIZE: [f32; 2] = [560.0, 390.0];
pub const HINT: &str = "提示：按 Ctrl + Enter 可快速產生。";

pub struct QrCodeMaker {
    pub form: Form,
    pub error_level: usize,
    pub box_size: u32,
    pub qr: Option<GrayImage>,
    pub generated_state: Option<(String, usize, u32)>,
    pub texture: Option<egui::TextureHandle>,
    pub status: String,
    pub show_version: bool,
    pub update: UpdateState,
    update_rx: Option<Receiver<Result<String, String>>>,
}

impl Default for QrCodeMaker {
    fn default() -> Self {
        Self {
            form: Form::default(),
            error_level: 1,
            box_size: 10,
            qr: None,
            generated_state: None,
            texture: None,
            status: HINT.into(),
            show_version: false,
            update: UpdateState::Idle,
            update_rx: None,
        }
    }
}

impl QrCodeMaker {
    pub fn has_current_qr(&self) -> bool {
        let Ok(payload) = self.form.payload() else { return false };
        self.qr.is_some()
            && self.generated_state.as_ref().is_some_and(|(generated, level, size)| {
                generated == &payload && *level == self.error_level && *size == self.box_size
            })
    }

    pub fn generate(&mut self, ctx: &egui::Context) {
        let payload = match self.form.payload() {
            Ok(payload) => payload,
            Err(message) => {
                self.status = message;
                return;
            }
        };
        match qr::render(&payload, self.error_level, self.box_size) {
            Ok(rendered) => {
                let image = &rendered.image;
                self.texture = Some(ctx.load_texture(
                    "qr-preview",
                    qr::to_color_image(image),
                    egui::TextureOptions::NEAREST,
                ));
                let preview_height = (image.height() as f32).min(640.0);
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                    560.0,
                    (preview_height + 360.0).max(560.0),
                )));
                let wifi = if self.form.mode == crate::payload::Mode::Wifi { "WiFi " } else { "" };
                self.status = format!("已產生{wifi}{0} × {0} QR code。", rendered.modules);
                self.qr = Some(rendered.image);
                self.generated_state = Some((payload, self.error_level, self.box_size));
            }
            Err(error) => self.status = format!("無法產生 QR code：{error}"),
        }
    }

    pub fn clear(&mut self, ctx: &egui::Context) {
        self.form.clear();
        self.qr = None;
        self.generated_state = None;
        self.texture = None;
        self.status = HINT.into();
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
            INITIAL_SIZE[0],
            INITIAL_SIZE[1],
        )));
    }

    pub fn save(&mut self) {
        if !self.has_current_qr() {
            return;
        }
        let Some(image) = self.qr.as_ref() else { return };
        match qr::save(image) {
            Ok(Some(path)) => self.status = format!("已儲存：{}", path.display()),
            Ok(None) => {}
            Err(error) => self.status = format!("儲存失敗：{error}"),
        }
    }

    // ---- 版本 / 更新 ----

    pub fn start_update_check(&mut self, ctx: &egui::Context) {
        if self.update == UpdateState::Checking {
            return;
        }
        self.update = UpdateState::Checking;
        self.update_rx = Some(update::spawn_check(ctx));
    }

    fn poll_update(&mut self) {
        let result = self.update_rx.as_ref().and_then(|rx| rx.try_recv().ok());
        if let Some(result) = result {
            self.update = update::state_from_result(result);
            self.update_rx = None;
        }
    }

    pub fn open_version_window(&mut self, ctx: &egui::Context) {
        self.show_version = true;
        if self.update == UpdateState::Idle {
            self.start_update_check(ctx);
        }
    }
}

impl eframe::App for QrCodeMaker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_update();
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter)) {
            self.generate(ctx);
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme::BACKGROUND).inner_margin(egui::Margin::same(18)))
            .show(ctx, |ui| self.main_panel(ui, ctx));
        self.version_window(ctx);
    }
}

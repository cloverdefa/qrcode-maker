//! 應用程式狀態與核心邏輯。

use crate::payload::Form;
use crate::update::{self, UpdateState};
use crate::{qr, theme};
use eframe::egui;
use image::GrayImage;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;

pub const INITIAL_SIZE: [f32; 2] = [560.0, 390.0];
pub const HINT: &str = "提示：按 Ctrl + Enter 可快速產生。";

type CheckRx = Receiver<Result<String, String>>;
type DownloadRx = (String, Receiver<Result<PathBuf, String>>);

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
    update_rx: Option<CheckRx>,
    download_rx: Option<DownloadRx>,
}

impl Default for QrCodeMaker {
    fn default() -> Self {
        // 清除上次更新留下的 .exe.old（失敗則忽略，下次啟動再試）。
        update::cleanup_old_version();
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
            download_rx: None,
        }
    }
}

impl QrCodeMaker {
    // ---- QR 產生 / 儲存 ----

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

    /// 是否有可用（或已下載）的新版本，用於標題列提示。
    pub fn has_update(&self) -> bool {
        matches!(
            self.update,
            UpdateState::Available(_) | UpdateState::Downloading(_) | UpdateState::Downloaded(..)
        )
    }

    /// 檢查或下載進行中，用於停用「重新檢查」。
    pub fn update_busy(&self) -> bool {
        matches!(self.update, UpdateState::Checking | UpdateState::Downloading(_))
    }

    pub fn start_update_check(&mut self, ctx: &egui::Context) {
        if self.update_busy() {
            return;
        }
        self.update = UpdateState::Checking;
        self.update_rx = Some(update::spawn_check(ctx));
    }

    pub fn start_download(&mut self, ctx: &egui::Context, tag: String) {
        if !update::CAN_DOWNLOAD || self.update_busy() {
            return;
        }
        self.download_rx = Some((tag.clone(), update::spawn_download(ctx, tag.clone())));
        self.update = UpdateState::Downloading(tag);
    }

    /// Windows：替換執行檔並關閉目前程式；其他平台：開啟下載位置。
    pub fn finish_update(&mut self, ctx: &egui::Context, path: &Path) {
        #[cfg(target_os = "windows")]
        match update::apply_update(path) {
            Ok(()) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            Err(error) => self.update = UpdateState::Failed(error),
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = ctx;
            update::reveal_in_folder(path);
        }
    }

    fn poll_update(&mut self) {
        let checked = self.update_rx.as_ref().and_then(|rx| rx.try_recv().ok());
        if let Some(result) = checked {
            self.update = update::state_from_check(result);
            self.update_rx = None;
        }

        let downloaded = self
            .download_rx
            .as_ref()
            .and_then(|(tag, rx)| rx.try_recv().ok().map(|result| (tag.clone(), result)));
        if let Some((tag, result)) = downloaded {
            self.update = update::state_from_download(tag, result);
            self.download_rx = None;
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

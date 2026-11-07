//! 介面繪製：主面板、各卡片、版本視窗。

use crate::app::QrCodeMaker;
use crate::payload::{Mode, SECURITY_NONE};
use crate::theme;
use crate::update::{self, UpdateState};
use eframe::egui::{self, Color32, RichText};

const EC_LEVELS: [&str; 4] = ["低 (L)", "中 (M)", "較高 (Q)", "高 (H)"];
const SECURITY_LABELS: [&str; 3] = ["WPA / WPA2 / WPA3", "WEP", "無密碼"];

fn colored_button(label: &str, size: f32, fill: Color32, stroke: Color32) -> egui::Button<'static> {
    egui::Button::new(RichText::new(label).size(size).strong().color(Color32::WHITE))
        .fill(fill)
        .stroke(egui::Stroke::new(1.0_f32, stroke))
}

impl QrCodeMaker {
    pub fn main_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let available = ui.available_size();
        let has_current_qr = self.has_current_qr();
        if has_current_qr && (available.x < 420.0 || available.y < 460.0) {
            self.compact_preview(ui, available);
            return;
        }
        self.header(ui, ctx);
        ui.add_space(14.0);
        self.content_card(ui);
        ui.add_space(10.0);
        self.settings_card(ui);
        ui.add_space(12.0);
        self.actions(ui, ctx, has_current_qr);
        if has_current_qr {
            ui.add_space(8.0);
            self.preview_card(ui);
        }
        ui.add_space(8.0);
        ui.label(RichText::new(&self.status).color(theme::MUTED));
    }

    fn compact_preview(&self, ui: &mut egui::Ui, available: egui::Vec2) {
        if let Some(texture) = &self.texture {
            let side = available.x.min(available.y).max(1.0);
            ui.centered_and_justified(|ui| {
                ui.image((texture.id(), egui::vec2(side, side)));
            });
        }
    }

    fn header(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("▦").size(28.0).strong().color(theme::ACCENT));
            ui.vertical(|ui| {
                ui.heading(RichText::new("QR Code Maker").size(24.0).strong().color(theme::TITLE));
                ui.label(RichText::new("快速建立文字、網址或 WiFi QR code").color(theme::MUTED));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let has_update = matches!(self.update, UpdateState::Available(_));
                let text = if has_update {
                    RichText::new(format!("v{} ● 有新版本", update::VERSION)).color(theme::WARN)
                } else {
                    RichText::new(format!("v{}", update::VERSION)).color(theme::MUTED)
                };
                if ui.button(text).clicked() {
                    self.open_version_window(ctx);
                }
            });
        });
    }

    fn content_card(&mut self, ui: &mut egui::Ui) {
        theme::card().show(ui, |ui| {
            let form = &mut self.form;
            ui.horizontal(|ui| {
                ui.label(RichText::new("內容類型").strong().color(theme::LABEL));
                ui.selectable_value(&mut form.mode, Mode::Text, "文字 / 網址");
                ui.selectable_value(&mut form.mode, Mode::Wifi, "WiFi");
            });
            ui.add_space(10.0);
            if form.mode == Mode::Text {
                ui.label(RichText::new("文字或網址").strong());
                ui.add(
                    egui::TextEdit::multiline(&mut form.text)
                        .desired_width(ui.available_width())
                        .desired_rows(4)
                        .hint_text("輸入要放進 QR code 的內容"),
                );
            } else {
                ui.label(RichText::new("WiFi 名稱 (SSID)").strong());
                ui.add(
                    egui::TextEdit::singleline(&mut form.ssid)
                        .desired_width(ui.available_width())
                        .hint_text("例如：Home WiFi"),
                );
                ui.add_space(6.0);
                ui.label(RichText::new("密碼").strong());
                ui.add(
                    egui::TextEdit::singleline(&mut form.password)
                        .desired_width(ui.available_width())
                        .password(!form.show_password)
                        .interactive(form.security != SECURITY_NONE)
                        .hint_text("輸入 WiFi 密碼"),
                );
                ui.horizontal(|ui| {
                    ui.checkbox(&mut form.show_password, "顯示密碼");
                    ui.checkbox(&mut form.hidden, "隱藏的網路");
                });
                ui.horizontal(|ui| {
                    ui.label("加密方式");
                    egui::ComboBox::from_id_salt("security")
                        .selected_text(SECURITY_LABELS[form.security])
                        .show_ui(ui, |ui| {
                            for (index, name) in SECURITY_LABELS.iter().enumerate() {
                                ui.selectable_value(&mut form.security, index, *name);
                            }
                        });
                });
            }
        });
    }

    fn settings_card(&mut self, ui: &mut egui::Ui) {
        theme::card().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("QR 設定").strong().color(theme::LABEL));
                ui.separator();
                ui.label("錯誤修正");
                egui::ComboBox::from_id_salt("error-level")
                    .selected_text(EC_LEVELS[self.error_level])
                    .show_ui(ui, |ui| {
                        for (index, name) in EC_LEVELS.iter().enumerate() {
                            ui.selectable_value(&mut self.error_level, index, *name);
                        }
                    });
                ui.label("模組大小");
                ui.add(egui::DragValue::new(&mut self.box_size).range(1..=40));
            });
        });
    }

    fn actions(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, has_current_qr: bool) {
        let input_ready = self.form.payload().is_ok();
        if self.generated_state.is_some() {
            let button_width = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;
            ui.horizontal(|ui| {
                let save = colored_button(
                    "另存 PNG / JPEG",
                    16.0,
                    Color32::from_rgb(35, 145, 75),
                    Color32::from_rgb(23, 110, 55),
                );
                if ui
                    .add_enabled(has_current_qr, save.min_size(egui::vec2(button_width, 44.0)))
                    .clicked()
                {
                    self.save();
                }
                let clear = colored_button("清除", 18.0, theme::ERROR, Color32::from_rgb(140, 30, 30));
                if ui.add_sized([button_width, 44.0], clear).clicked() {
                    self.clear(ctx);
                }
            });
        } else if input_ready {
            let generate = colored_button(
                "產生 QR Code",
                18.0,
                Color32::from_rgb(20, 112, 190),
                Color32::from_rgb(12, 83, 145),
            );
            if ui.add_sized([ui.available_width(), 44.0], generate).clicked() {
                self.generate(ctx);
            }
        }
    }

    fn preview_card(&self, ui: &mut egui::Ui) {
        theme::card().show(ui, |ui| {
            let remaining = ui.available_size();
            ui.set_min_size(egui::vec2(ui.available_width(), remaining.y.max(160.0)));
            if let Some(texture) = &self.texture {
                let available = ui.available_size();
                let side = texture.size_vec2().x.min(available.x).min(available.y).max(1.0);
                ui.centered_and_justified(|ui| {
                    ui.image((texture.id(), egui::vec2(side, side)));
                });
            }
        });
    }

    pub fn version_window(&mut self, ctx: &egui::Context) {
        if !self.show_version {
            return;
        }
        let mut open = true;
        egui::Window::new("版本資訊")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label(RichText::new(format!("目前版本：v{}", update::VERSION)).strong());
                ui.add_space(6.0);
                match self.update.clone() {
                    UpdateState::Idle => {}
                    UpdateState::Checking => {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("正在檢查更新…");
                        });
                    }
                    UpdateState::UpToDate => {
                        ui.label(RichText::new("已是最新版本。").color(Color32::from_rgb(35, 145, 75)));
                    }
                    UpdateState::Available(tag) => {
                        ui.label(RichText::new(format!("有新版本：{tag}")).color(theme::WARN).strong());
                        if ui.button("前往下載頁面").clicked() {
                            update::open_url(update::RELEASES_URL);
                        }
                    }
                    UpdateState::Failed(error) => {
                        ui.label(RichText::new(error).color(theme::ERROR));
                    }
                }
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let checking = self.update == UpdateState::Checking;
                    if ui.add_enabled(!checking, egui::Button::new("重新檢查")).clicked() {
                        self.start_update_check(ctx);
                    }
                    if ui.button("GitHub").clicked() {
                        update::open_url("https://github.com/cloverdefa/qrcode-maker");
                    }
                });
            });
        self.show_version = open;
    }
}

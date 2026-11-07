//! 顏色、字型、全域樣式。

use eframe::egui::{self, Color32};

pub const BACKGROUND: Color32 = Color32::from_rgb(243, 246, 251);
pub const MUTED: Color32 = Color32::from_rgb(103, 117, 138);
pub const LABEL: Color32 = Color32::from_rgb(58, 72, 94);
pub const TITLE: Color32 = Color32::from_rgb(32, 46, 66);
pub const ACCENT: Color32 = Color32::from_rgb(30, 111, 196);
pub const WARN: Color32 = Color32::from_rgb(190, 110, 0);
pub const ERROR: Color32 = Color32::from_rgb(190, 48, 48);

pub fn card() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(224, 230, 239)))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::same(14))
}

pub fn setup(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "noto_sans_tc".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSansTC.ttf")).into(),
    );
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        fonts.families.entry(family).or_default().push("noto_sans_tc".to_owned());
    }
    ctx.set_fonts(fonts);
    ctx.set_visuals(egui::Visuals::light());

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.visuals.selection.bg_fill = Color32::from_rgb(198, 226, 252);
    style.visuals.selection.stroke = egui::Stroke::new(1.0_f32, ACCENT);
    ctx.set_style(style);
}

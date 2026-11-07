#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod app;
mod payload;
mod qr;
mod theme;
mod ui;
mod update;

use eframe::egui;

fn load_icon() -> egui::IconData {
    let icon = image::load_from_memory(include_bytes!("../assets/app-icon.png"))
        .expect("embedded app icon must be valid")
        .into_rgba8();
    let (width, height) = icon.dimensions();
    egui::IconData { rgba: icon.into_raw(), width, height }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("qrcode-maker")
            .with_inner_size(app::INITIAL_SIZE)
            .with_min_inner_size([220.0, 260.0])
            .with_resizable(true)
            .with_icon(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "QR Code Maker",
        options,
        Box::new(|cc| {
            theme::setup(&cc.egui_ctx);
            Ok(Box::<app::QrCodeMaker>::default())
        }),
    )
}

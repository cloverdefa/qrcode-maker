//! QR code 產生、轉換為 egui 圖片、儲存檔案。

use eframe::egui;
use image::{DynamicImage, GrayImage, ImageFormat, Luma};
use qrcode::{EcLevel, QrCode};
use std::path::{Path, PathBuf};

pub struct Rendered {
    pub image: GrayImage,
    /// QR code 每邊的模組數（不含留白）。
    pub modules: u32,
}

pub fn render(payload: &str, level: usize, box_size: u32) -> Result<Rendered, String> {
    let levels = [EcLevel::L, EcLevel::M, EcLevel::Q, EcLevel::H];
    let code = QrCode::with_error_correction_level(payload.as_bytes(), levels[level])
        .map_err(|e| e.to_string())?;
    let modules = code.width() as u32;
    let image = code
        .render::<Luma<u8>>()
        .quiet_zone(true)
        .module_dimensions(box_size, box_size)
        .build();
    Ok(Rendered { image, modules })
}

pub fn to_color_image(image: &GrayImage) -> egui::ColorImage {
    let rgba: Vec<u8> = image.pixels().flat_map(|p| [p[0], p[0], p[0], 255]).collect();
    egui::ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize], &rgba)
}

/// 開啟另存新檔對話框並寫檔。使用者取消時回傳 `Ok(None)`。
pub fn save(image: &GrayImage) -> Result<Option<PathBuf>, String> {
    let Some(path) = rfd::FileDialog::new()
        .set_title("儲存 QR code")
        .set_file_name("qrcode.png")
        .add_filter("PNG 圖片", &["png"])
        .add_filter("JPEG 圖片", &["jpg", "jpeg"])
        .save_file()
    else {
        return Ok(None);
    };
    let format = match Path::new(&path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("jpg" | "jpeg") => ImageFormat::Jpeg,
        _ => ImageFormat::Png,
    };
    DynamicImage::ImageLuma8(image.clone())
        .save_with_format(&path, format)
        .map_err(|e| e.to_string())?;
    Ok(Some(path))
}

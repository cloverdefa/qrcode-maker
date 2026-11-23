//! 版本檢查與更新下載：以 curl 存取 GitHub Releases 網址（不使用 API，避開速率限制）。
//! Windows 下隱藏 CLI 視窗。

use eframe::egui;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RELEASES_URL: &str = "https://github.com/cloverdefa/qrcode-maker/releases/latest";
const DOWNLOAD_BASE: &str = "https://github.com/cloverdefa/qrcode-maker/releases/download";

/// Release 資產檔名（請依實際發佈的檔名調整）。
#[cfg(target_os = "windows")]
const ASSET_NAME: &str = "qrcode-maker-windows-x86_64.zip";
#[cfg(target_os = "macos")]
const ASSET_NAME: &str = "qrcode-maker-macos-aarch64.zip";
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
const ASSET_NAME: &str = "qrcode-maker-linux-x86_64.tar.gz";

#[cfg(windows)]
const NULL_DEVICE: &str = "NUL";
#[cfg(not(windows))]
const NULL_DEVICE: &str = "/dev/null";

#[derive(Clone, PartialEq)]
pub enum UpdateState {
    Idle,
    Checking,
    UpToDate,
    Available(String),
    Failed(String),
}

/// 在背景執行緒檢查最新版本，結果由 Receiver 回傳（最新 tag）。
pub fn spawn_check(ctx: &egui::Context) -> Receiver<Result<String, String>> {
    let (tx, rx) = mpsc::channel();
    let ctx = ctx.clone();
    thread::spawn(move || {
        let _ = tx.send(fetch_latest());
        ctx.request_repaint();
    });
    rx
}

/// 在背景執行緒下載指定 tag 的更新檔，結果為下載後的檔案路徑。
pub fn spawn_download(ctx: &egui::Context, tag: String) -> Receiver<Result<PathBuf, String>> {
    let (tx, rx) = mpsc::channel();
    let ctx = ctx.clone();
    thread::spawn(move || {
        let _ = tx.send(download_asset(&tag));
        ctx.request_repaint();
    });
    rx
}

pub fn state_from_result(result: Result<String, String>) -> UpdateState {
    match result {
        Ok(tag) if is_newer(&tag, VERSION) => UpdateState::Available(tag),
        Ok(_) => UpdateState::UpToDate,
        Err(error) => UpdateState::Failed(error),
    }
}

/// 追蹤 /releases/latest 的轉址，從最終網址（.../releases/tag/vX.Y.Z）取得 tag。
fn fetch_latest() -> Result<String, String> {
    let mut cmd = Command::new("curl");
    cmd.args([
        "-fsSL",
        "--max-time",
        "10",
        "-H",
        "User-Agent: qrcode-maker",
        "-o",
        NULL_DEVICE,
        "-w",
        "%{url_effective}",
        RELEASES_URL,
    ]);
    hide_console(&mut cmd);
    let output = cmd.output().map_err(|e| format!("無法執行 curl：{e}"))?;
    if !output.status.success() {
        return Err("無法連線到 GitHub，請稍後再試。".into());
    }
    parse_tag(String::from_utf8_lossy(&output.stdout).trim())
        .ok_or_else(|| "無法解析最新版本資訊。".into())
}

/// 由最終網址解析 tag，例如 https://github.com/.../releases/tag/v1.2.3 → v1.2.3
fn parse_tag(url: &str) -> Option<String> {
    let rest = &url[url.find("/releases/tag/")? + "/releases/tag/".len()..];
    let tag = rest.split(['/', '?', '#']).next()?.trim();
    (!tag.is_empty()).then(|| tag.to_owned())
}

pub fn download_url(tag: &str) -> String {
    format!("{DOWNLOAD_BASE}/{tag}/{ASSET_NAME}")
}

fn download_asset(tag: &str) -> Result<PathBuf, String> {
    let dest = std::env::temp_dir().join(format!("qrcode-maker-{tag}-{ASSET_NAME}"));
    let mut cmd = Command::new("curl");
    cmd.arg("-fSL")
        .args(["--max-time", "300"])
        .args(["-H", "User-Agent: qrcode-maker"])
        .arg("-o")
        .arg(&dest)
        .arg(download_url(tag));
    hide_console(&mut cmd);
    let output = cmd.output().map_err(|e| format!("無法執行 curl：{e}"))?;
    if !output.status.success() {
        let _ = std::fs::remove_file(&dest);
        return Err("下載更新失敗，請稍後再試。".into());
    }
    Ok(dest)
}

fn parse_version(value: &str) -> Vec<u64> {
    value
        .trim()
        .trim_start_matches(['v', 'V'])
        .split(['.', '-', '+'])
        .map_while(|part| part.parse::<u64>().ok())
        .collect()
}

fn is_newer(latest: &str, current: &str) -> bool {
    let (mut a, mut b) = (parse_version(latest), parse_version(current));
    let len = a.len().max(b.len());
    a.resize(len, 0);
    b.resize(len, 0);
    a > b
}

pub fn open_url(url: &str) {
    #[cfg(target_os = "windows")]
    let mut cmd = Command::new("explorer");
    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open");
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    let mut cmd = Command::new("xdg-open");
    cmd.arg(url);
    hide_console(&mut cmd);
    let _ = cmd.spawn();
}

fn hide_console(cmd: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(windows))]
    let _ = cmd;
}

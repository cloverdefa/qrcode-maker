//! 版本檢查：以 curl 查詢 GitHub Releases（Windows 下隱藏 CLI 視窗）。

use eframe::egui;
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const API_URL: &str = "https://api.github.com/repos/cloverdefa/qrcode-maker/releases/latest";
pub const RELEASES_URL: &str = "https://github.com/cloverdefa/qrcode-maker/releases/latest";

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

pub fn state_from_result(result: Result<String, String>) -> UpdateState {
    match result {
        Ok(tag) if is_newer(&tag, VERSION) => UpdateState::Available(tag),
        Ok(_) => UpdateState::UpToDate,
        Err(error) => UpdateState::Failed(error),
    }
}

fn fetch_latest() -> Result<String, String> {
    let mut cmd = Command::new("curl");
    cmd.args([
        "-fsSL",
        "--max-time",
        "10",
        "-H",
        "User-Agent: qrcode-maker",
        "-H",
        "Accept: application/vnd.github+json",
        API_URL,
    ]);
    hide_console(&mut cmd);
    let output = cmd.output().map_err(|e| format!("無法執行 curl：{e}"))?;
    if !output.status.success() {
        return Err("無法連線到 GitHub，請稍後再試。".into());
    }
    parse_tag(&String::from_utf8_lossy(&output.stdout)).ok_or_else(|| "無法解析最新版本資訊。".into())
}

fn parse_tag(json: &str) -> Option<String> {
    let rest = &json[json.find("\"tag_name\"")? + "\"tag_name\"".len()..];
    let rest = &rest[rest.find('"')? + 1..];
    Some(rest[..rest.find('"')?].to_owned())
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

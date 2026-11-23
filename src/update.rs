//! 版本檢查與更新下載。
//!
//! - 不使用 GitHub API（避開速率限制），僅存取一般 github.com 網址。
//! - 版本判斷：追蹤 `/releases/latest` 的 302 轉址，由最終網址解析 tag。
//! - 更新下載：`/releases/download/<tag>/<資產檔名>` 直連。
//! - 以 curl 執行，Windows 下隱藏 CLI 視窗。

use eframe::egui;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{self, Receiver};
use std::thread;

// ───────────────────────── 常數 ─────────────────────────

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RELEASES_URL: &str = "https://github.com/cloverdefa/qrcode-maker/releases/latest";
const DOWNLOAD_BASE: &str = "https://github.com/cloverdefa/qrcode-maker/releases/download";
const USER_AGENT: &str = "User-Agent: qrcode-maker";

/// Release 資產檔名（需與 release.yml 上傳的名稱一致）。
#[cfg(target_os = "windows")]
const ASSET_NAME: &str = "QRCodeMaker.exe";
#[cfg(not(target_os = "windows"))]
const ASSET_NAME: &str = "qrcode-maker-linux-x86_64.tar.gz";

/// 此平台是否有對應的 Release 資產可下載（目前 macOS 沒有）。
pub const CAN_DOWNLOAD: bool = cfg!(any(target_os = "windows", target_os = "linux"));

#[cfg(windows)]
const NULL_DEVICE: &str = "NUL";
#[cfg(not(windows))]
const NULL_DEVICE: &str = "/dev/null";

// ───────────────────────── 狀態 ─────────────────────────

#[derive(Clone, PartialEq)]
pub enum UpdateState {
    Idle,
    Checking,
    UpToDate,
    /// 有新版本（tag）。
    Available(String),
    /// 下載中（tag）。
    Downloading(String),
    /// 下載完成（tag, 檔案路徑）。
    Downloaded(String, PathBuf),
    Failed(String),
}

pub fn state_from_check(result: Result<String, String>) -> UpdateState {
    match result {
        Ok(tag) if is_newer(&tag, VERSION) => UpdateState::Available(tag),
        Ok(_) => UpdateState::UpToDate,
        Err(error) => UpdateState::Failed(error),
    }
}

pub fn state_from_download(tag: String, result: Result<PathBuf, String>) -> UpdateState {
    match result {
        Ok(path) => UpdateState::Downloaded(tag, path),
        Err(error) => UpdateState::Failed(error),
    }
}

// ───────────────────────── 背景工作 ─────────────────────────

/// 在背景執行緒執行 `job`，完成後透過 Receiver 回傳並要求重繪。
fn spawn_job<T, F>(ctx: &egui::Context, job: F) -> Receiver<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    let ctx = ctx.clone();
    thread::spawn(move || {
        let _ = tx.send(job());
        ctx.request_repaint();
    });
    rx
}

/// 檢查最新版本，回傳最新 tag（例如 `v0.6.6`）。
pub fn spawn_check(ctx: &egui::Context) -> Receiver<Result<String, String>> {
    spawn_job(ctx, fetch_latest)
}

/// 下載指定 tag 的更新檔，回傳下載後的檔案路徑。
pub fn spawn_download(ctx: &egui::Context, tag: String) -> Receiver<Result<PathBuf, String>> {
    spawn_job(ctx, move || download_asset(&tag))
}

// ───────────────────────── curl ─────────────────────────

fn curl() -> Command {
    let mut cmd = Command::new("curl");
    cmd.args(["-fsSL", "--proto", "=https", "-H", USER_AGENT]);
    hide_console(&mut cmd);
    cmd
}

/// 追蹤 `/releases/latest` 的轉址，由最終網址取得 tag。
fn fetch_latest() -> Result<String, String> {
    let output = curl()
        .args(["--max-time", "10", "-o", NULL_DEVICE, "-w", "%{url_effective}"])
        .arg(RELEASES_URL)
        .output()
        .map_err(|e| format!("無法執行 curl：{e}"))?;
    if !output.status.success() {
        return Err("無法連線到 GitHub，請稍後再試。".into());
    }
    parse_tag(String::from_utf8_lossy(&output.stdout).trim())
        .ok_or_else(|| "無法解析最新版本資訊。".into())
}

/// `https://github.com/.../releases/tag/v1.2.3` → `v1.2.3`
fn parse_tag(url: &str) -> Option<String> {
    const MARK: &str = "/releases/tag/";
    let rest = &url[url.find(MARK)? + MARK.len()..];
    let tag = rest.split(['/', '?', '#']).next()?.trim();
    (!tag.is_empty()).then(|| tag.to_owned())
}

pub fn download_url(tag: &str) -> String {
    format!("{DOWNLOAD_BASE}/{tag}/{ASSET_NAME}")
}

/// 下載到 `%TEMP%/qrcode-maker-<tag>/<資產檔名>`；先寫入 `.part` 並驗證檔頭後才改名。
fn download_asset(tag: &str) -> Result<PathBuf, String> {
    let dir = std::env::temp_dir().join(format!("qrcode-maker-{tag}"));
    fs::create_dir_all(&dir).map_err(|e| format!("無法建立暫存資料夾：{e}"))?;
    let dest = dir.join(ASSET_NAME);
    let part = dir.join(format!("{ASSET_NAME}.part"));

    let result = curl()
        .args(["--max-time", "300", "-o"])
        .arg(&part)
        .arg(download_url(tag))
        .output();

    let ok = matches!(&result, Ok(o) if o.status.success());
    if !ok {
        let _ = fs::remove_file(&part);
        return Err(match result {
            Err(e) => format!("無法執行 curl：{e}"),
            Ok(_) => "下載更新失敗，請稍後再試。".into(),
        });
    }

    if !has_expected_magic(&part) {
        let _ = fs::remove_file(&part);
        return Err("下載的檔案格式不正確。".into());
    }

    let _ = fs::remove_file(&dest);
    fs::rename(&part, &dest).map_err(|e| format!("無法儲存更新檔：{e}"))?;
    Ok(dest)
}

/// 簡單驗證檔頭：Windows exe 為 `MZ`，Linux tar.gz 為 gzip `1F 8B`。
fn has_expected_magic(path: &Path) -> bool {
    let mut head = [0u8; 2];
    let Ok(mut file) = fs::File::open(path) else {
        return false;
    };
    if file.read_exact(&mut head).is_err() {
        return false;
    }
    if cfg!(windows) {
        head == *b"MZ"
    } else {
        head == [0x1F, 0x8B]
    }
}

// ───────────────────────── 套用更新 ─────────────────────────

/// Windows：將執行中的 exe 改名為 `.exe.old`，複製新版就位並啟動。
/// 成功後呼叫端應關閉目前程式（例如 `ViewportCommand::Close`）。
#[cfg(target_os = "windows")]
pub fn apply_update(downloaded: &Path) -> Result<(), String> {
    let current = std::env::current_exe().map_err(|e| format!("無法取得目前程式路徑：{e}"))?;
    let old = old_exe_path(&current);

    let _ = fs::remove_file(&old);
    fs::rename(&current, &old).map_err(|e| format!("無法備份舊版：{e}"))?;
    if let Err(e) = fs::copy(downloaded, &current) {
        let _ = fs::rename(&old, &current); // 失敗則還原
        return Err(format!("無法寫入新版：{e}"));
    }

    let mut cmd = Command::new(&current);
    hide_console(&mut cmd);
    cmd.spawn().map_err(|e| format!("無法啟動新版：{e}"))?;
    Ok(())
}

/// 啟動時清除上次更新留下的舊檔。
#[cfg(target_os = "windows")]
pub fn cleanup_old_version() {
    if let Ok(current) = std::env::current_exe() {
        let _ = fs::remove_file(old_exe_path(&current));
    }
}

#[cfg(target_os = "windows")]
fn old_exe_path(current: &Path) -> PathBuf {
    current.with_extension("exe.old")
}

/// 非 Windows 不自動替換，無需清理。
#[cfg(not(target_os = "windows"))]
pub fn cleanup_old_version() {}

/// 非 Windows：在檔案管理員開啟下載資料夾（Linux 的 tar.gz 內含 install.sh，由使用者自行安裝）。
#[cfg(not(target_os = "windows"))]
pub fn reveal_in_folder(path: &Path) {
    if let Some(dir) = path.parent() {
        open_url(&dir.to_string_lossy());
    }
}

// ───────────────────────── 版本比較 ─────────────────────────

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

// ───────────────────────── 系統相關 ─────────────────────────

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

// ───────────────────────── 測試 ─────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tag_from_final_url() {
        assert_eq!(
            parse_tag("https://github.com/cloverdefa/qrcode-maker/releases/tag/v0.6.6"),
            Some("v0.6.6".into())
        );
        assert_eq!(
            parse_tag("https://github.com/x/y/releases/tag/v1.0.0?foo=1"),
            Some("v1.0.0".into())
        );
        assert_eq!(parse_tag("https://github.com/x/y/releases"), None);
    }

    #[test]
    fn compares_versions() {
        assert!(is_newer("v0.6.7", "0.6.6"));
        assert!(is_newer("v0.10.0", "0.9.9"));
        assert!(is_newer("v1.0", "0.9.9"));
        assert!(!is_newer("v0.6.6", "0.6.6"));
        assert!(!is_newer("v0.6.5", "0.6.6"));
    }

    #[test]
    fn builds_download_url() {
        let url = download_url("v0.6.6");
        assert!(url.starts_with(
            "https://github.com/cloverdefa/qrcode-maker/releases/download/v0.6.6/"
        ));
        assert!(url.ends_with(ASSET_NAME));
    }
}

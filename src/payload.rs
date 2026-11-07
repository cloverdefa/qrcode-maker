//! 輸入內容 → QR code 字串（文字 / 網址 / WiFi）。

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Text,
    Wifi,
}

/// 與 UI 下拉選單順序對應：WPA、WEP、無密碼。
const SECURITY_KEYS: [&str; 3] = ["WPA", "WEP", "nopass"];
pub const SECURITY_NONE: usize = 2;

pub struct Form {
    pub mode: Mode,
    pub text: String,
    pub ssid: String,
    pub password: String,
    pub security: usize,
    pub hidden: bool,
    pub show_password: bool,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            mode: Mode::Text,
            text: String::new(),
            ssid: String::new(),
            password: String::new(),
            security: 0,
            hidden: false,
            show_password: false,
        }
    }
}

impl Form {
    pub fn clear(&mut self) {
        self.text.clear();
        self.ssid.clear();
        self.password.clear();
    }

    pub fn payload(&self) -> Result<String, String> {
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
        let security = SECURITY_KEYS[self.security];
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
}

fn wifi_escape(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        if ['\\', ';', ',', ':', '"'].contains(&ch) {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    if !escaped.is_empty() && escaped.chars().all(|ch| ch.is_ascii_hexdigit()) {
        format!("\"{escaped}\"")
    } else {
        escaped
    }
}

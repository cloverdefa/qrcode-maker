use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/app-icon.ico");

    // 只有 Windows 主機上才有 winresource，且目標平台也要是 Windows
    #[cfg(windows)]
    if env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/app-icon.ico");
        res.compile().expect("embed Windows icon");
    }
}

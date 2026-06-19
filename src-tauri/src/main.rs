// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // ponytail: removes Gatekeeper quarantine from .app bundle on launch.
    // User may still need right-click → Open first time, but auto-update
    // and subsequent launches work without terminal commands.
    #[cfg(target_os = "macos")]
    if let Ok(exe) = std::env::current_exe() {
        if let Some(app_bundle) = exe.parent().and_then(|p| p.parent()) {
            std::process::Command::new("xattr")
                .args(["-dr", "com.apple.quarantine", app_bundle.to_str().unwrap_or("")])
                .output()
                .ok();
        }
    }
    app_lib::run();
}

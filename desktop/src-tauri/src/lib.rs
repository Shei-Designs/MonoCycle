// MonoCycle — single-cycle waveform designer for hardware samplers.
// Copyright (C) 2026  Andrew O'Shei  <https://github.com/AOShei>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// The version shown in the About panel. Single-sourced from Cargo.toml so a
/// release only ever means bumping the manifests, never the markup.
#[tauri::command]
fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Writes raw bytes to a user-supplied path. Used by the WAV exporter:
/// the frontend pops a native save dialog (tauri-plugin-dialog) to get a
/// user-blessed path, then hands the bytes here. Keeping this as an explicit
/// command lets us avoid pulling in tauri-plugin-fs and its scope/permission
/// system for the single thing we actually need to do with the filesystem.
#[tauri::command]
fn save_wav(path: String, bytes: Vec<u8>) -> Result<(), String> {
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // WebKitGTK's DMA-BUF renderer crashes on some Linux/Wayland setups
    // (notably Arch with webkit2gtk-4.1 >= 2.50). Force the legacy renderer
    // before WebKit initializes. Respects user override if already set.
    #[cfg(target_os = "linux")]
    unsafe {
        if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![app_version, save_wav])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

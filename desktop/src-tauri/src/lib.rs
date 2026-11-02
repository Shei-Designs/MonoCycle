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

use tauri::{Manager, PhysicalSize, WebviewWindow};

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

/// The configured window size is the layout's natural size, which is taller
/// than many laptop screens. Shrink (never grow) the window so it fits the
/// monitor's work area with a little margin. The frontend scales the UI to
/// whatever size the window ends up.
///
/// Best effort only: if the monitor or sizes can't be read it does nothing.
/// Two things make the numbers unreliable at startup, before the window is on
/// screen: outer size is often reported as 0 (so title-bar height is unknown),
/// and GNOME on Wayland reports the whole monitor as the work area, top bar
/// included. Hence the fixed allowance for title bar + desktop panel.
fn fit_window_to_monitor(win: &WebviewWindow) {
    let monitor = match win.current_monitor() {
        Ok(Some(m)) => m,
        _ => match win.primary_monitor() {
            Ok(Some(m)) => m,
            _ => return,
        },
    };
    let (Ok(inner), Ok(outer)) = (win.inner_size(), win.outer_size()) else {
        return;
    };
    let area = monitor.work_area().size;
    let scale = monitor.scale_factor();
    let chrome_w = outer.width.saturating_sub(inner.width);
    let chrome_h = outer
        .height
        .saturating_sub(inner.height)
        .max((90.0 * scale) as u32);
    let max_w = ((area.width as f64 * 0.95) as u32).saturating_sub(chrome_w);
    let max_h = ((area.height as f64 * 0.95) as u32).saturating_sub(chrome_h);
    if max_w == 0 || max_h == 0 || (inner.width <= max_w && inner.height <= max_h) {
        return;
    }
    let _ = win.set_size(PhysicalSize::new(
        inner.width.min(max_w),
        inner.height.min(max_h),
    ));
    let _ = win.center();
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
        .setup(|app| {
            // Don't start the window hidden to do this ("visible": false +
            // show()): on GNOME/Wayland a window shown that way gains its
            // title-bar height a second time, ends up taller than the screen,
            // and its close button lands under the top bar where it can't be
            // clicked.
            if let Some(win) = app.get_webview_window("main") {
                fit_window_to_monitor(&win);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

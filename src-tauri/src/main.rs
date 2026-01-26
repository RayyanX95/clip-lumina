mod commands;
mod history;
mod models;
mod state;

use arboard::Clipboard;
use chrono::Utc;
use std::sync::atomic::Ordering;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    ActivationPolicy, Emitter, Manager,
};
use uuid::Uuid;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;
use image::ColorType;
use image::ImageEncoder;
use std::io::Cursor;
use urlencoding;

use crate::commands::*;
use crate::history::{load_history, save_history};
use crate::models::ClipItem;
use crate::state::{IGNORE_NEXT_CLIP, LAST_CLICK};

/// Specialized macOS helper to retrieve the actual file path when a file is copied in Finder.
/// Standard clipboard libraries often only catch the filename or a small icon.
#[cfg(target_os = "macos")]
fn get_macos_file_path() -> Option<String> {
    use std::process::Command;
    let output = Command::new("osascript")
        .arg("-e")
        .arg("get posix path of (the clipboard as «class furl»)")
        .output()
        .ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() && std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }
    None
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Hide the application icon from the macOS Dock (runs as a background "Accessory" app)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            // Spawn a background thread to continuously monitor the system clipboard
            std::thread::spawn(move || {
                let mut clipboard = match Clipboard::new() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to initialize clipboard listener: {}", e);
                        return;
                    }
                };

                let mut last_content = String::new();

                loop {
                    // Poll the clipboard every 300ms to detect changes
                    std::thread::sleep(std::time::Duration::from_millis(300));

                    let mut new_content = String::new();
                    let mut kind = "text";

                    let image_extensions = ["png", "jpg", "jpeg", "gif", "webp", "svg", "ico"];

                    // 1. Try to get macOS file path first (for Files copied in Finder)
                    #[cfg(target_os = "macos")]
                    if let Some(path) = get_macos_file_path() {
                        let extension = std::path::Path::new(&path)
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        if image_extensions.contains(&extension.as_str()) {
                            new_content = path;
                            kind = "file";
                        } else {
                            // Non-image file detected. Skip adding to history.
                            if path != last_content {
                                last_content = path;
                            }
                            continue;
                        }
                    }

                    // 2. Try reading standard text content (if no image file path was found)
                    if new_content.is_empty() {
                        if let Ok(text) = clipboard.get_text() {
                            if !text.is_empty() {
                                let mut path_str = text.trim();
                                if path_str.starts_with("file://") {
                                    path_str = path_str.trim_start_matches("file://");
                                }

                                // Decode URL-encoded paths (e.g. My%20Image.png -> My Image.png)
                                let decoded_path = urlencoding::decode(path_str)
                                    .map(|s| s.into_owned())
                                    .unwrap_or_else(|_| path_str.to_string());

                                let path = std::path::Path::new(&decoded_path);
                                if path.is_absolute() && path.exists() {
                                    let extension = path
                                        .extension()
                                        .and_then(|ext| ext.to_str())
                                        .unwrap_or("")
                                        .to_lowercase();

                                    if image_extensions.contains(&extension.as_str()) {
                                        new_content = decoded_path;
                                        kind = "file";
                                    } else {
                                        // Skip non-image file paths
                                        if decoded_path != last_content {
                                            last_content = decoded_path;
                                        }
                                        continue;
                                    }
                                } else {
                                    // It's text. Check if it looks like a filename for a non-image file.
                                    let text_trimmed = text.trim();
                                    let lower_text = text_trimmed.to_lowercase();
                                    let non_image_exts = [
                                        ".pdf", ".zip", ".tar", ".gz", ".7z", ".rar", ".dmg",
                                        ".pkg", ".exe", ".docx", ".xlsx", ".pptx",
                                    ];

                                    // Heuristic: if it's a single word and ends with a common non-image extension, skip it.
                                    if non_image_exts.iter().any(|ext| lower_text.ends_with(ext))
                                        && !text_trimmed.contains(' ')
                                    {
                                        if text != last_content {
                                            last_content = text.to_string();
                                        }
                                        continue;
                                    }

                                    new_content = text;
                                    kind = "text";
                                }
                            }
                        }
                    }

                    // 3. Try reading raw image data (e.g. from browser or Preview)
                    if new_content.is_empty() {
                        if let Ok(img) = clipboard.get_image() {
                            let mut buffer = Vec::new();
                            let width = img.width as u32;
                            let height = img.height as u32;

                            if width > 0 && height > 0 {
                                let mut cursor = Cursor::new(&mut buffer);
                                let encoder = image::codecs::png::PngEncoder::new(&mut cursor);

                                if let Ok(_) = encoder.write_image(
                                    &img.bytes,
                                    width,
                                    height,
                                    ColorType::Rgba8.into(),
                                ) {
                                    let b64 = BASE64.encode(&buffer);
                                    new_content = format!("data:image/png;base64,{}", b64);
                                    kind = "image";
                                }
                            }
                        }
                    }

                    // If there's new content and it's different from the last thing we saw
                    if !new_content.is_empty() && new_content != last_content {
                        last_content = new_content.clone();

                        let mut should_ignore = false;
                        // Check if this content was just written BY our app (prevents infinite loop when we copy something)
                        if let Ok(mut ignore_guard) = IGNORE_NEXT_CLIP.lock() {
                            if let Some(ignored) = ignore_guard.as_ref() {
                                if *ignored == new_content {
                                    should_ignore = true;
                                    *ignore_guard = None;
                                }
                            }
                        }

                        let mut history = load_history(&app_handle);

                        // Only add to history if it's not explicitly ignored and not identical to the current top item
                        if !should_ignore
                            && history
                                .first()
                                .map(|i| i.content != new_content)
                                .unwrap_or(true)
                        {
                            let new_item = ClipItem {
                                id: Uuid::new_v4().to_string(),
                                content: new_content.clone(),
                                timestamp: Utc::now().timestamp_millis(),
                                pinned: false,
                                kind: kind.to_string(),
                            };

                            history.insert(0, new_item);

                            // Limit history to 50 items, but preserve pinned items wherever they are
                            if history.len() > 50 {
                                if let Some(idx) = history.iter().rposition(|i| !i.pinned) {
                                    history.remove(idx);
                                } else {
                                    history.truncate(50);
                                }
                            }

                            save_history(&app_handle, &history);

                            // Notify the frontend to refresh its list
                            if let Err(e) = app_handle.emit("clipboard://update", &history) {
                                eprintln!("Failed to emit clipboard update: {}", e);
                            }
                        }
                    }
                }
            });

            // Configure the System Tray (Menu Bar icon)
            let show_i = MenuItem::with_id(app, "show", "Show ClipLumina", true, None::<&str>)?;
            let clear_i = MenuItem::with_id(app, "clear", "Clear History", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &clear_i, &quit_i])?;

            let tray_icon = tauri::image::Image::from_path("icons/tray-icon.png")
                .unwrap_or_else(|_| app.default_window_icon().unwrap().clone());

            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .icon_as_template(true)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "clear" => {
                        let mut history = load_history(app);
                        history.retain(|item| item.pinned);
                        save_history(app, &history);
                        let _ = app.emit("clipboard://update", &history);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        let now = Utc::now().timestamp_millis();
                        let last = LAST_CLICK.load(Ordering::Relaxed);
                        // Debounce: prevent multiple clicks from toggling the window too fast (300ms threshold)
                        if now - last < 300 {
                            return;
                        }
                        LAST_CLICK.store(now, Ordering::Relaxed);

                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let is_visible = window.is_visible().unwrap_or(false);

                            if is_visible {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Auto-hide the main window whenever it loses focus (clicks outside the app)
            if let Some(window) = app.get_webview_window("main") {
                let w_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(focused) = event {
                        if !focused {
                            let _ = w_clone.hide();
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_history,
            delete_clip,
            copy_to_clip,
            toggle_pin_clip,
            clear_history,
            get_current_clip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

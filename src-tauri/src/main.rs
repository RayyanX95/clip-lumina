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

use crate::commands::*;
use crate::history::{load_history, save_history};
use crate::models::ClipItem;
use crate::state::{IGNORE_NEXT_CLIP, LAST_CLICK};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            // Setup Background Clipboard Listener
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
                    std::thread::sleep(std::time::Duration::from_millis(300));

                    let mut new_content = String::new();
                    let mut kind = "text";

                    if let Ok(text) = clipboard.get_text() {
                        if !text.is_empty() {
                            new_content = text;
                            kind = "text";
                        }
                    }

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

                    if !new_content.is_empty() && new_content != last_content {
                        last_content = new_content.clone();

                        let mut should_ignore = false;
                        if let Ok(mut ignore_guard) = IGNORE_NEXT_CLIP.lock() {
                            if let Some(ignored) = ignore_guard.as_ref() {
                                if *ignored == new_content {
                                    should_ignore = true;
                                    *ignore_guard = None;
                                }
                            }
                        }

                        let mut history = load_history(&app_handle);

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

                            if history.len() > 50 {
                                if let Some(idx) = history.iter().rposition(|i| !i.pinned) {
                                    history.remove(idx);
                                } else {
                                    history.truncate(50);
                                }
                            }

                            save_history(&app_handle, &history);

                            if let Err(e) = app_handle.emit("clipboard://update", &history) {
                                eprintln!("Failed to emit clipboard update: {}", e);
                            }
                        }
                    }
                }
            });

            // Setup System Tray
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

            // Window Behavior (Hide on Blur)
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

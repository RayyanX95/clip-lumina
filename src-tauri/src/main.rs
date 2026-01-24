use arboard::Clipboard;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Mutex;
static LAST_CLICK: AtomicI64 = AtomicI64::new(0);
static IGNORE_NEXT_CLIP: Mutex<Option<String>> = Mutex::new(None);

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    ActivationPolicy, AppHandle, Emitter, Manager, Runtime,
};
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClipItem {
    id: String,
    content: String,
    timestamp: i64,
    #[serde(default)]
    pinned: bool,
}

const STORE_PATH: &str = "history.json";

fn load_history<R: Runtime>(app: &AppHandle<R>) -> Vec<ClipItem> {
    let store = app.store(STORE_PATH).expect("failed to get store");
    if let Some(val) = store.get("history") {
        serde_json::from_value(val).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_history<R: Runtime>(app: &AppHandle<R>, history: &Vec<ClipItem>) {
    let store = app.store(STORE_PATH).expect("failed to get store");
    store.set("history", json!(history));
    let _ = store.save(); // Persist to disk
}

#[tauri::command]
fn get_history(app: AppHandle) -> Vec<ClipItem> {
    load_history(&app)
}

#[tauri::command]
fn delete_clip(app: AppHandle, id: String) -> Vec<ClipItem> {
    let mut history = load_history(&app);
    history.retain(|item| item.id != id);
    save_history(&app, &history);
    let _ = app.emit("clipboard://update", &history);
    history
}

#[tauri::command]
fn toggle_pin_clip(app: AppHandle, id: String) -> Vec<ClipItem> {
    let mut history = load_history(&app);
    if let Some(item) = history.iter_mut().find(|i| i.id == id) {
        item.pinned = !item.pinned;
    }
    save_history(&app, &history);
    let _ = app.emit("clipboard://update", &history);
    history
}

#[tauri::command]
fn clear_history(app: AppHandle) -> Vec<ClipItem> {
    let mut history = load_history(&app);
    // Keep only pinned items
    history.retain(|item| item.pinned);
    save_history(&app, &history);
    let _ = app.emit("clipboard://update", &history);
    history
}

#[tauri::command]
fn copy_to_clip(content: String) -> Result<(), String> {
    // Set ignore flag BEFORE writing to clipboard
    if let Ok(mut ignore) = IGNORE_NEXT_CLIP.lock() {
        println!("Setting ignore for: '{}'", content.replace("\n", "\\n"));
        *ignore = Some(content.clone());
    }

    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(content).map_err(|e| e.to_string())
}

// Kept for manual "Read Now" or debugging
#[tauri::command]
fn get_current_clip() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.get_text().unwrap_or_else(|_| "".to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // 1. Hide Dock Icon (macOS)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            // 2. Setup Background Clipboard Listener
            std::thread::spawn(move || {
                let mut clipboard = match Clipboard::new() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to initialize clipboard listener: {}", e);
                        return;
                    }
                };

                let mut last_content = clipboard.get_text().unwrap_or_default();

                loop {
                    std::thread::sleep(std::time::Duration::from_millis(300));

                    if let Ok(content) = clipboard.get_text() {
                        if content != last_content && !content.is_empty() {
                            last_content = content.clone();

                            // Dedup: if newly copied item is same as most recent, ignore
                            let mut should_ignore = false;

                            // Check ignore flag
                            if let Ok(mut ignore_guard) = IGNORE_NEXT_CLIP.lock() {
                                if let Some(ignored) = ignore_guard.as_ref() {
                                    // Compare both trimming whitespace to be safe
                                    if ignored.trim() == content.trim() || *ignored == content {
                                        should_ignore = true;
                                        *ignore_guard = None; // Clear it once matched
                                    }
                                }
                            }

                            // Load history first for dedup check
                            let mut history = load_history(&app_handle);

                            // Dedup: check ignore flag AND store content
                            if !should_ignore
                                && history
                                    .first()
                                    .map(|i| i.content != content)
                                    .unwrap_or(true)
                            {
                                // Load, Append, Save
                                // The history is already loaded above, no need to load again here.

                                let new_item = ClipItem {
                                    id: Uuid::new_v4().to_string(),
                                    content: content.clone(),
                                    timestamp: Utc::now().timestamp_millis(),
                                    pinned: false,
                                };

                                // Insert at top
                                history.insert(0, new_item);

                                // Truncate to 50
                                if history.len() > 50 {
                                    if let Some(idx) = history.iter().rposition(|i| !i.pinned) {
                                        history.remove(idx);
                                    } else {
                                        history.truncate(50);
                                    }
                                }

                                save_history(&app_handle, &history);

                                // Emit update
                                if let Err(e) = app_handle.emit("clipboard://update", &history) {
                                    eprintln!("Failed to emit clipboard update: {}", e);
                                }
                            }
                        }
                    }
                }
            });

            // 3. Setup System Tray
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id() == "quit" {
                        app.exit(0);
                    }
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

            // 4. Window Behavior (Hide on Blur)
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
            get_current_clip,
            get_history,
            delete_clip,
            copy_to_clip,
            toggle_pin_clip,
            clear_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

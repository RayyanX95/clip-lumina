use crate::history::{load_history, save_history};
use crate::models::ClipItem;
use crate::state::IGNORE_NEXT_CLIP;
use arboard::Clipboard;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn get_history(app: AppHandle) -> Vec<ClipItem> {
    load_history(&app)
}

#[tauri::command]
pub fn delete_clip(app: AppHandle, id: String) -> Vec<ClipItem> {
    let mut history = load_history(&app);
    history.retain(|item| item.id != id);
    save_history(&app, &history);
    let _ = app.emit("clipboard://update", &history);
    history
}

#[tauri::command]
pub fn toggle_pin_clip(app: AppHandle, id: String) -> Vec<ClipItem> {
    let mut history = load_history(&app);
    if let Some(item) = history.iter_mut().find(|i| i.id == id) {
        item.pinned = !item.pinned;
    }
    save_history(&app, &history);
    let _ = app.emit("clipboard://update", &history);
    history
}

#[tauri::command]
pub fn clear_history(app: AppHandle) -> Vec<ClipItem> {
    let mut history = load_history(&app);
    history.retain(|item| item.pinned);
    save_history(&app, &history);
    let _ = app.emit("clipboard://update", &history);
    history
}

#[tauri::command]
pub fn copy_to_clip(content: String, kind: String) -> Result<(), String> {
    if let Ok(mut ignore) = IGNORE_NEXT_CLIP.lock() {
        *ignore = Some(content.clone());
    }

    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;

    if kind == "image" || content.starts_with("data:image") {
        let b64_part = if content.contains(",") {
            content.split(',').nth(1).unwrap_or("")
        } else {
            &content
        };

        let bytes = BASE64.decode(b64_part).map_err(|e| e.to_string())?;
        let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        let image_data = arboard::ImageData {
            width: width as usize,
            height: height as usize,
            bytes: std::borrow::Cow::Borrowed(rgba.as_raw()),
        };

        clipboard.set_image(image_data).map_err(|e| e.to_string())
    } else {
        clipboard.set_text(content).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn get_current_clip() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.get_text().unwrap_or_else(|_| "".to_string())
}

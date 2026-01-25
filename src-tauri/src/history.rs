use crate::models::ClipItem;
use serde_json::json;
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

pub const STORE_PATH: &str = "history.json";

pub fn load_history<R: Runtime>(app: &AppHandle<R>) -> Vec<ClipItem> {
    let store = app.store(STORE_PATH).expect("failed to get store");
    if let Some(val) = store.get("history") {
        serde_json::from_value(val).unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save_history<R: Runtime>(app: &AppHandle<R>, history: &Vec<ClipItem>) {
    let store = app.store(STORE_PATH).expect("failed to get store");
    store.set("history", json!(history));
    let _ = store.save(); // Persist to disk
}

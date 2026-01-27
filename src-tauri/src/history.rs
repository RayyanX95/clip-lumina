use crate::models::{ClipItem, HistoryWrapper};
use serde_json::json;
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

pub const STORE_PATH: &str = "history.json";
pub const CURRENT_STORAGE_VERSION: u32 = 1;

pub fn load_history<R: Runtime>(app: &AppHandle<R>) -> Vec<ClipItem> {
    let store = app.store(STORE_PATH).expect("failed to get store");
    if let Some(val) = store.get("history") {
        // Try to parse as the new versioned wrapper first
        if let Ok(wrapper) = serde_json::from_value::<HistoryWrapper>(val.clone()) {
            return wrapper.items;
        }
        // Fallback for legacy "Version 0" data (which was just a raw Vec<ClipItem>)
        serde_json::from_value::<Vec<ClipItem>>(val).unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save_history<R: Runtime>(app: &AppHandle<R>, history: &Vec<ClipItem>) {
    let store = app.store(STORE_PATH).expect("failed to get store");
    let wrapper = HistoryWrapper {
        version: CURRENT_STORAGE_VERSION,
        items: history.clone(),
    };
    store.set("history", json!(wrapper));
    let _ = store.save(); // Persist to disk
}

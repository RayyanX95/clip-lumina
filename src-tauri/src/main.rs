use arboard::Clipboard;

#[tauri::command]
fn get_current_clip() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.get_text().unwrap_or_else(|_| "".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_current_clip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

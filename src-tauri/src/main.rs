use arboard::Clipboard;

#[tauri::command]
fn get_current_clip() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.get_text().unwrap_or_else(|_| "".to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Emitter; // Ensure Emitter trait is in scope
            let app_handle = app.handle().clone();

            std::thread::spawn(move || {
                let mut clipboard = match Clipboard::new() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to initialize clipboard listener: {}", e);
                        return;
                    }
                };

                // Initialize with current content to avoid re-triggering immediately
                // or start empty to trigger on first loop.
                // Let's start empty so the UI gets the current clip automatically on connect if needed
                // OR better: read first, so we don't spam. The UI can fetch initial state.
                let mut last_content = clipboard.get_text().unwrap_or_default();

                loop {
                    // Poll interval
                    std::thread::sleep(std::time::Duration::from_millis(500));

                    if let Ok(content) = clipboard.get_text() {
                        if content != last_content && !content.is_empty() {
                            last_content = content.clone();
                            // Emit event to frontend
                            if let Err(e) = app_handle.emit("clipboard://change", &last_content) {
                                eprintln!("Failed to emit clipboard event: {}", e);
                            } else {
                                println!("Clipboard changed: {:.20}...", last_content);
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_current_clip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

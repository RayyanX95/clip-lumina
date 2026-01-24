use arboard::Clipboard;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    ActivationPolicy, Manager,
};

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
                    std::thread::sleep(std::time::Duration::from_millis(500));

                    if let Ok(content) = clipboard.get_text() {
                        if content != last_content && !content.is_empty() {
                            last_content = content.clone();
                            if let Err(e) = app_handle.emit("clipboard://change", &last_content) {
                                eprintln!("Failed to emit clipboard event: {}", e);
                            } else {
                                println!("Clipboard changed: {:.20}...", last_content);
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
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_current_clip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

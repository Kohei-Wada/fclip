use crate::clipboard::ClipboardWatcher;
use crate::commands::{self, AppState};
use crate::config::Config;
use crate::db::Database;
use crate::search::FuzzySearcher;
use std::sync::Arc;
use tauri::{App, Manager};

fn toggle_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let visible = window.is_visible().unwrap_or(false);
        if visible {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn setup_tray(app: &App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use tauri::menu::{MenuBuilder, MenuItemBuilder};
    use tauri::tray::TrayIconBuilder;

    let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

    let app_handle_tray = app.handle().clone();
    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "show" => toggle_window(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(move |_tray, event| {
            if let tauri::tray::TrayIconEvent::DoubleClick { button, .. } = event {
                if button == tauri::tray::MouseButton::Left {
                    toggle_window(&app_handle_tray);
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn setup_global_shortcut(
    app: &App,
    hotkey: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, _shortcut, event| {
                if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                    toggle_window(&app_handle);
                }
            })
            .build(),
    )?;

    use tauri_plugin_global_shortcut::GlobalShortcutExt;
    eprintln!("[fclip] Registering hotkey: {}", hotkey);
    match hotkey.parse::<tauri_plugin_global_shortcut::Shortcut>() {
        Ok(shortcut) => match app.handle().global_shortcut().register(shortcut) {
            Ok(_) => eprintln!("[fclip] Hotkey registered successfully"),
            Err(e) => eprintln!("[fclip] WARNING: Failed to register hotkey: {}", e),
        },
        Err(e) => eprintln!("[fclip] WARNING: Invalid hotkey '{}': {}", hotkey, e),
    }

    Ok(())
}

pub fn run() {
    let config = Config::load();

    let db_path = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(crate::constants::APP_DIR_NAME)
        .join(crate::constants::DB_FILENAME);

    let db = Arc::new(Database::new(&db_path).expect("Failed to initialize database"));

    let max_history = config.behavior.max_history;
    let autostart = config.behavior.autostart;
    let hotkey = config.hotkey.open.clone();

    let state = AppState {
        db: Arc::clone(&db),
        config,
        searcher: FuzzySearcher::new(),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::search_clipboard,
            commands::paste_entry,
            commands::delete_entry,
            commands::toggle_pin,
            commands::get_keybindings,
            commands::get_theme,
            commands::open_config,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .setup(move |app| {
            setup_tray(app)?;
            setup_global_shortcut(app, &hotkey)?;

            use tauri_plugin_autostart::ManagerExt;
            let autostart_manager = app.autolaunch();
            if autostart {
                let _ = autostart_manager.enable();
                eprintln!("[fclip] Autostart enabled");
            } else {
                let _ = autostart_manager.disable();
            }

            let app_handle = app.handle().clone();

            let watcher = Arc::new(ClipboardWatcher::new(db, max_history));
            watcher.start(app_handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

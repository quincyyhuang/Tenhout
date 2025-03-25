// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use appdata::{get_subtitle, AppData, TENHOU_URLS};
use std::sync::Mutex;
use tauri::menu::{Menu, MenuItem, SubmenuBuilder};
use tauri::Manager;
mod appdata;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Try to read app settings from file
            let app_config_dir = app
                .path()
                .app_config_dir()
                .expect("Cannot get app config directory");

            let app_config_path = app_config_dir.join("config.json");

            let app_settings = match std::fs::read_to_string(&app_config_path) {
                Ok(config_file) => match serde_json::from_str(&config_file) {
                    Ok(data) => data,
                    Err(_) => AppData::default(),
                },
                // Not config file found or error reading it
                Err(_) => AppData::default(),
            };

            let url = tauri::Url::parse(&app_settings.main_page_url).unwrap();

            // Make window subtitle
            let subtitle = get_subtitle(&url.as_str());

            // Handle window initialization
            let webview_window =
                tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::External(url))
                    .build()?;

            webview_window.set_size(tauri::LogicalSize {
                width: app_settings.window_width.clone(),
                height: app_settings.window_height.clone(),
            })?;
            webview_window.set_title(format!("Tenhout - {subtitle}").as_str())?;

            // Set up app state management
            let app_settings = Mutex::new(app_settings);
            app.manage(app_settings);

            // Handle window events
            let app_handle = webview_window.app_handle().clone();
            let scale_factor = webview_window.scale_factor().ok().unwrap();
            webview_window.on_window_event(move |event| {
                let app_settings = app_handle.state::<Mutex<AppData>>();
                let mut app_settings = app_settings.lock().unwrap();

                match event {
                    // When window is resized, save the new window size
                    tauri::WindowEvent::Resized(size) => {
                        let logical_size: tauri::LogicalSize<u32> = size.to_logical(scale_factor);
                        app_settings.window_width = logical_size.width;
                        app_settings.window_height = logical_size.height;
                    }
                    // When window is closing, save the window size to config file
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        match serde_json::to_string(&*app_settings) {
                            Ok(config) => {
                                // Ensure config directory exists
                                let _ = std::fs::create_dir_all(&app_config_dir);
                                let _ = std::fs::write(&app_config_path.as_path(), &config);
                            }
                            Err(_) => {}
                        };
                    }
                    _ => {}
                }
            });

            // Set up app menu
            let app_menu = Menu::new(app)?;
            let submenu = SubmenuBuilder::new(app, "Tenhout")
                .item(&MenuItem::with_id(
                    app,
                    "tenhou-4k",
                    "Tenhou Web 4K",
                    true,
                    Some("CommandOrControl+1"),
                )?)
                .item(&MenuItem::with_id(
                    app,
                    "tenhou-old",
                    "Tenhou Web",
                    true,
                    Some("CommandOrControl+2"),
                )?)
                .item(&MenuItem::with_id(
                    app,
                    "tenhou-pairi",
                    "Tenhou Pairi",
                    true,
                    Some("CommandOrControl+3"),
                )?)
                .separator()
                .item(&MenuItem::with_id(
                    app,
                    "ron2",
                    "Ron2",
                    true,
                    Some("CommandOrControl+4"),
                )?)
                .separator()
                .item(&MenuItem::with_id(
                    app,
                    "join-lobby",
                    "Join Lobby",
                    true,
                    Some("CommandOrControl+J"),
                )?)
                .separator()
                .item(&MenuItem::with_id(
                    app,
                    "clear-cache",
                    "Clear All Cache",
                    true,
                    Some("CommandOrControl+Shift+C"),
                )?)
                .separator()
                .quit()
                .build()?;

            let file_submenu = SubmenuBuilder::new(app, "File")
                .cut()
                .copy()
                .paste()
                .select_all()
                .build()?;

            app_menu.append(&submenu)?;
            app_menu.append(&file_submenu)?;
            let _ = app.set_menu(app_menu);

            // Show window
            webview_window.show()?;
            Ok(())
        })
        .on_menu_event(|app, event| match event.id().0.as_str() {
            "tenhou-4k" => {
                let url = tauri::Url::parse(TENHOU_URLS[2]).unwrap();

                // Save the URL in appdata
                let app_settings = app.state::<Mutex<AppData>>();
                let mut app_settings = app_settings.lock().unwrap();
                app_settings.main_page_url = url.as_str().to_owned();

                if let Some(main_window) = app.get_webview_window("main") {
                    let _ = main_window.navigate(url.clone());
                    let subtitle = get_subtitle(&url.as_str());
                    let _ = main_window.set_title(format!("Tenhou - {subtitle}").as_str());
                }
            }
            "tenhou-old" => {
                let url = tauri::Url::parse(TENHOU_URLS[1]).unwrap();

                // Save the URL in appdata
                let app_settings = app.state::<Mutex<AppData>>();
                let mut app_settings = app_settings.lock().unwrap();
                app_settings.main_page_url = url.as_str().to_owned();

                if let Some(main_window) = app.get_webview_window("main") {
                    let _ = main_window.navigate(url.clone());
                    let subtitle = get_subtitle(&url.as_str());
                    let _ = main_window.set_title(format!("Tenhou - {subtitle}").as_str());
                }
            }
            "tenhou-pairi" => {
                let url = tauri::Url::parse(TENHOU_URLS[0]).unwrap();
                if let Ok(pairi_window) =
                    tauri::WebviewWindowBuilder::new(app, "pairi", tauri::WebviewUrl::External(url))
                        .build()
                {
                    let _ = pairi_window.set_size(tauri::LogicalSize {
                        width: 400,
                        height: 600,
                    });
                    let _ = pairi_window.set_title("Tenhout - Pairi");
                    let _ = pairi_window.show();
                }
            }
            "ron2" => {
                let url = tauri::Url::parse(TENHOU_URLS[3]).unwrap();

                // Save the URL in appdata
                let app_settings = app.state::<Mutex<AppData>>();
                let mut app_settings = app_settings.lock().unwrap();
                app_settings.main_page_url = url.as_str().to_owned();

                if let Some(main_window) = app.get_webview_window("main") {
                    let _ = main_window.navigate(url.clone());
                    let subtitle = get_subtitle(&url.as_str());
                    let _ = main_window.set_title(format!("Tenhou - {subtitle}").as_str());
                }
            }
            "join-lobby" => {
                if let Some(main_window) = app.get_webview_window("main") {
                    // Load join-lobby.js from resource directory and evaluate it
                    let resource_path = app
                        .path()
                        .resolve(
                            "plugins/join-lobby.js",
                            tauri::path::BaseDirectory::Resource,
                        )
                        .expect("join-lobby.js file does not exist");
                    let join_lobby_js = std::fs::read_to_string(&resource_path)
                        .expect("Failed to read join-lobby.js");
                    // main_window.open_devtools();
                    let _ = main_window.eval(join_lobby_js.as_str());
                }
            }
            "clear-cache" => {
                if let Some(main_window) = app.get_webview_window("main") {
                    let _ = main_window.clear_all_browsing_data();
                    // Reload after clearing cache
                    let _ = main_window.eval("window.location.reload()");
                }
            }
            _ => {}
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

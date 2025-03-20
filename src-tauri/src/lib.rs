// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use appdata::AppData;
use std::sync::Mutex;
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

            // Handle window initialization
            let webview_window =
                tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::External(url))
                    .build()?;

            webview_window.set_size(tauri::LogicalSize {
                width: app_settings.window_width.clone(),
                height: app_settings.window_height.clone(),
            })?;
            webview_window.set_title("Tenhout")?;

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

            // Show window
            webview_window.show()?;
            Ok(())
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

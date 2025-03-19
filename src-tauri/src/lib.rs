// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod appdata;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut app_data = appdata::AppData {
        window_width: 800,
        window_height: 600,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

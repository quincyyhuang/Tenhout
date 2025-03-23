use serde::{Deserialize, Serialize};

pub const TENHOU_URLS: [&str; 4] = [
    "https://tenhou.net/2/",
    "https://tenhou.net/3/",
    "https://tenhou.net/4/",
    "https://ron2.jp/3/",
];

pub fn get_subtitle(url: &str) -> String {
    let index = TENHOU_URLS
        .iter()
        .position(|&t| t == url)
        .expect("URL not found");
    match index {
        1 => "Tenhou Web".to_string(),
        2 => "Tenhou Web 4K".to_string(),
        3 => "Ron2".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub window_width: u32,
    pub window_height: u32,
    pub main_page_url: String,
}

impl AppData {
    pub fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            main_page_url: TENHOU_URLS[2].to_owned(),
        }
    }
}

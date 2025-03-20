use serde::{Deserialize, Serialize};

pub const TENHOU_URLS: [&str; 3] = [
    "https://tenhou.net/2/",
    "https://tenhou.net/3/",
    "https://tenhou.net/4/",
];

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

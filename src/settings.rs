use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub volume: f32,
    pub resolution: String,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            volume: 1.0,
            resolution: "1920x1080".to_string(),
        }
    }

    pub fn load_from_file(file_path: &str) -> Self {
        let data = fs::read_to_string(file_path).unwrap_or_else(|_| String::new());
        serde_json::from_str(&data).unwrap_or_else(|_| Settings::new())
    }

    pub fn save_to_file(&self, file_path: &str) {
        let data = serde_json::to_string(self).unwrap();
        fs::write(file_path, data).expect("Unable to write file");
    }
}
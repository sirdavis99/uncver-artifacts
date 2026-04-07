use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PersistentState {
    pub input_text: String,
}

impl PersistentState {
    pub fn load() -> Self {
        if let Some(mut path) = dirs::data_dir() {
            path.push("uncver-artifacts");
            path.push("search_state.json");
            
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(state) = serde_json::from_str(&content) {
                        return state;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(mut path) = dirs::data_dir() {
            path.push("uncver-artifacts");
            let _ = fs::create_dir_all(&path);
            path.push("search_state.json");
            
            if let Ok(content) = serde_json::to_string(self) {
                let _ = fs::write(path, content);
            }
        }
    }
}

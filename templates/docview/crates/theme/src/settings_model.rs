use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsModel {
    pub font_size: f32,
    pub line_height: f32,
}

impl Default for SettingsModel {
    fn default() -> Self {
        Self {
            font_size: 13.0,
            line_height: 1.4,
        }
    }
}


use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub observe_after_track: bool,
    pub view_text_width: usize,
    pub date_format: String,
    date_small_format: String,
    #[serde(default)]
    pub default_observe_group: String,
    #[serde(default)]
    pub default_day_duration: i32,
    #[serde(default)]
    pub default_month_duration: i32,
    #[serde(default)]
    pub default_migrate_duration: i32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            observe_after_track: true,
            view_text_width: 12,
            date_format: "%m/%d/%Y".to_string(),
            date_small_format: "%m/%d".to_string(),
            default_observe_group: "day".to_string(),
            default_day_duration: 7,
            default_month_duration: 1,
            default_migrate_duration: 30,
        }
    }

    pub fn get_date_format(&self) -> &String {
        let date_format = if self.view_text_width > 10 {
            &self.date_format
        } else {
            &self.date_small_format
        };

        date_format
    }
}

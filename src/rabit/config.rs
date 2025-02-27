use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub view_text_width: usize,
    pub date_format: String,
    date_small_format: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            view_text_width: 12,
            date_format: "%m/%d/%Y".to_string(),
            date_small_format: "%m/%d".to_string(),
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

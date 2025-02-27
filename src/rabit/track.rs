use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub date: String,
    pub time: String,
    pub value: String,
}

impl Track {
    pub fn new(config: &Config, value: &Option<String>) -> Self {
        let date_now = chrono::Local::now();
        let date = format!("{}", date_now.format(&config.date_format));
        let date_time = date_now.time();
        let time = format!("{}", date_time.format("%H:%M:%S"));
        let mut value_str = "\u{1f5f9}".to_string();
        if let Some(value) = value {
            value_str = value.to_string();
        }

        Self {
            date,
            time,
            value: value_str,
        }
    }
}

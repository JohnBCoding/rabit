use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub date: String,
    pub time: String,
}

impl Track {
    pub fn new(config: &Config) -> Self {
        let date_now = chrono::Local::now();
        let date = format!("{}", date_now.format(&config.date_format));
        let date_time = date_now.time();
        let time = format!("{}", date_time.format("%H:%M:%S"));
        Self { date, time }
    }
}

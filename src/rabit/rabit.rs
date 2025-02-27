use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rabit {
    pub name: String,
    pub init_date: String,
    pub init_time: String,
    pub tracks: Vec<Track>,
}

impl Rabit {
    pub fn new(config: &Config, name: &str) -> Self {
        let date_now = chrono::Local::now();
        let init_date = format!("{}", date_now.format(config.get_date_format()));
        let date_time = date_now.time();
        let init_time = format!("{}", date_time.format("%H:%M:%S"));
        Self {
            name: name.to_string(),
            init_date,
            init_time,
            tracks: vec![Track::new(config)],
        }
    }
}

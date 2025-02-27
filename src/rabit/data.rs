use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub config: Config,
    pub rabits: Vec<Rabit>,
}

impl Data {
    pub fn default() -> Self {
        Self {
            config: Config::new(),
            rabits: vec![],
        }
    }

    pub fn track(&mut self, new_rabit: Rabit, overwrite_track: bool) {
        let mut has_rabit = None;
        for rabit in &mut self.rabits {
            if rabit.name == new_rabit.name {
                has_rabit = Some(rabit);
                break;
            }
        }

        if let Some(rabit) = has_rabit {
            let date_now = chrono::Local::now();
            let date_now_str = format!("{}", date_now.format(self.config.get_date_format()));
            let mut track_index = None;
            for (index, track) in rabit.tracks.iter().enumerate() {
                if track.date == date_now_str {
                    track_index = Some(index);
                    break;
                }
            }

            if let Some(track_index) = track_index {
                if overwrite_track {
                    rabit.tracks[track_index] = Track::new(&self.config);
                } else {
                    eprintln!("There is already a track for today, use --backtrack to retrack.")
                }
            } else {
                rabit.tracks.push(Track::new(&self.config));
            }
        } else {
            // !TODO prompt user if they want to create or not
            self.rabits.push(new_rabit);
        }
    }

    pub fn cull_rabit(&mut self, rabit_name: &str) {
        let mut cull_index = 0;
        for (index, rabit) in self.rabits.iter().enumerate() {
            if rabit.name == rabit_name {
                cull_index = index;
                break;
            }
        }

        self.rabits.remove(cull_index);
    }

    pub fn print_fluffle(&self) {
        let text_width = self.config.view_text_width;
        let date_format = self.config.get_date_format();

        let mut date_strs = vec![];
        let mut date_cmp_strs = vec![];
        let mut rabit_line_str = "".to_string();

        let date_now = chrono::Local::now();
        let mut start_date = date_now.date_naive() - chrono::Days::new(7);
        while start_date <= date_now.date_naive() {
            let date_str = format!("{:^text_width$}", start_date.format(&date_format));
            date_strs.push(date_str);
            let date_cmp_str = format!("{}", start_date.format(&self.config.date_format));
            date_cmp_strs.push(date_cmp_str);
            start_date = start_date + chrono::Days::new(1);
        }

        self.rabits.iter().for_each(|rabit| {
            let mut track_line = vec![];
            for date_str in &date_cmp_strs {
                let mut found_track = false;
                for track in &rabit.tracks {
                    if track.date == *date_str {
                        found_track = true;
                        break;
                    }
                }

                if found_track {
                    track_line.push("\u{1f5f9}");
                } else {
                    track_line.push("\u{2610}");
                }
            }

            let track_str = track_line
                .iter()
                .map(|line| format!("{:^text_width$}", line))
                .collect::<String>();

            rabit_line_str = format!(
                "{}{:<text_width$}|{:^text_width$}\n",
                rabit_line_str,
                format!("{:.text_width$}", &rabit.name),
                track_str
            );
        });
        println!(
            "{:<text_width$}{:<text_width$}",
            "Rabit",
            date_strs
                .iter()
                .map(|str| { format!("{:<text_width$}", str) })
                .collect::<String>()
        );
        println!("{:-<text_width$}", "");
        println!("{}{:-<text_width$}", rabit_line_str, "");
    }

    pub fn print_rabit(&self, name: &String) {}
}

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

    pub fn track(&mut self, new_rabit: Rabit, value: &Option<String>, overwrite_track: bool) {
        let mut has_rabit = None;
        for rabit in &mut self.rabits {
            if rabit.name == new_rabit.name {
                has_rabit = Some(rabit);
                break;
            }
        }

        if let Some(rabit) = has_rabit {
            let date_now = Local::now();
            let date_now_str = format!("{}", date_now.format(&self.config.date_format));
            let mut track_index = None;
            for (index, track) in rabit.tracks.iter().enumerate() {
                if track.date == date_now_str {
                    track_index = Some(index);
                    break;
                }
            }

            if let Some(track_index) = track_index {
                if overwrite_track {
                    rabit.tracks[track_index] = Track::new(&self.config, value);
                } else {
                    eprintln!("There is already a track for today, use --backtrack to retrack.")
                }
            } else {
                rabit.tracks.push(Track::new(&self.config, value));
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

    fn print_fluffle_by_day(&self, rabit: &Rabit, set_duration: &Option<i32>) {
        let duration = if let Some(duration) = set_duration {
            *duration
        } else {
            self.config.default_day_duration
        };

        let text_width = self.config.view_text_width;
        let header_width = text_width * 2;
        let date_format = self.config.get_date_format();

        println!("\n{:^header_width$}", rabit.name);
        println!("{:-<header_width$}", "");
        let mut date_strs = vec![];
        let mut date_strs_cmp = vec![];
        let date_now = Local::now();
        for i in (0..duration).rev() {
            let day = max(0, (date_now.day() as i32) - i);

            if let Some(start_date) =
                NaiveDate::from_ymd_opt(date_now.year(), date_now.month(), day as u32)
            {
                date_strs.push(format!("{:^text_width$}", start_date.format(&date_format)));
                date_strs_cmp.push(format!("{}", start_date.format(&self.config.date_format)));

                if date_strs.len() == 7 || i == 0 {
                    let date_str = date_strs.iter().map(|str| str.clone()).collect::<String>();
                    println!("{}", date_str);
                    let mut track_line_str = "".to_string();
                    for date_str in &date_strs_cmp {
                        let mut found_track = false;
                        for track in &rabit.tracks {
                            if track.date == *date_str {
                                found_track = true;
                                track_line_str = format!(
                                    "{}{:^text_width$}",
                                    track_line_str,
                                    track.value.as_str()
                                );
                                break;
                            }
                        }

                        if !found_track {
                            track_line_str =
                                format!("{}{:^text_width$}", track_line_str, "\u{2610}");
                        }
                    }
                    println!("{}\n", track_line_str);

                    date_strs = vec![];
                    date_strs_cmp = vec![];
                }
            }
        }
    }

    fn print_fluffle_by_month(&self, rabit: &Rabit, set_duration: &Option<i32>) {
        let duration = if let Some(duration) = set_duration {
            *duration
        } else {
            self.config.default_month_duration
        };

        let text_width = self.config.view_text_width;
        let header_width = text_width * 2;
        let date_format = self.config.get_date_format();

        println!("\n{:^header_width$}", rabit.name);
        println!("{:-<header_width$}", "");
        let date_now = Local::now();
        for i in (0..duration).rev() {
            let mut year = date_now.year();
            let mut month = (date_now.month() as i32) - i;

            if month < 0 {
                year -= 1;
                month = 12 + month;
            }

            if let Some(mut start_date) = NaiveDate::from_ymd_opt(year, month as u32, 1) {
                println!("{:^header_width$}", start_date.format("%B"));
                println!("{:-<header_width$}", "");
                let mut date_strs = vec![];
                let mut date_strs_cmp = vec![];
                loop {
                    let current_month = start_date.month0();
                    date_strs.push(format!("{:^text_width$}", start_date.format(&date_format)));
                    date_strs_cmp.push(format!("{}", start_date.format(&self.config.date_format)));
                    start_date = start_date + Days::new(1);

                    if date_strs.len() == 7 || start_date.month0() != current_month {
                        let date_str = date_strs.iter().map(|str| str.clone()).collect::<String>();
                        println!("{}", date_str);
                        let mut track_line_str = "".to_string();
                        for date_str in &date_strs_cmp {
                            let mut found_track = false;
                            for track in &rabit.tracks {
                                if track.date == *date_str {
                                    found_track = true;
                                    track_line_str = format!(
                                        "{}{:^text_width$}",
                                        track_line_str,
                                        track.value.as_str()
                                    );
                                    break;
                                }
                            }

                            if !found_track {
                                track_line_str =
                                    format!("{}{:^text_width$}", track_line_str, "\u{2610}");
                            }
                        }
                        println!("{}\n", track_line_str);

                        date_strs = vec![];
                        date_strs_cmp = vec![];
                    }

                    if start_date.month0() != current_month {
                        break;
                    }
                }
            }
        }
    }

    pub fn print_fluffle(&self, set_group: &Option<String>, duration: &Option<i32>) {
        let group = if let Some(group) = set_group {
            group
        } else {
            self.config.default_observe_group.as_str()
        };

        self.rabits.iter().for_each(|rabit| match group {
            "day" => {
                self.print_fluffle_by_day(&rabit, duration);
            }
            "month" => {
                self.print_fluffle_by_month(&rabit, duration);
            }
            _ => {}
        });
    }

    pub fn print_rabit(&self, name: &String, set_group: &Option<String>, duration: &Option<i32>) {
        let group = if let Some(group) = set_group {
            group
        } else {
            self.config.default_observe_group.as_str()
        };

        let mut rabit = None;
        for drabit in &self.rabits {
            if &drabit.name == name {
                rabit = Some(drabit);
                break;
            }
        }

        if let Some(rabit) = rabit {
            match group {
                "day" => {
                    self.print_fluffle_by_day(&rabit, duration);
                }
                "month" => {
                    self.print_fluffle_by_month(&rabit, duration);
                }
                _ => {}
            }
        }
    }

    pub fn to_csv(&self, filename: &str, duration: &Option<i32>) {
        let mut csv_string = "".to_string();
        let duration = if let Some(duration) = duration {
            *duration
        } else {
            self.config.default_migrate_duration
        };

        let mut start_date = Local::now() - Days::new(duration as u64);
        let mut date_strs = vec![];
        for _ in 1..=duration + 1 {
            date_strs.push(format!("{}", start_date.format(&self.config.date_format)));
            start_date = start_date + Days::new(1);
        }

        date_strs.iter().for_each(|date_str| {
            let mut added_track_for_date = false;
            self.rabits.iter().for_each(|rabit| {
                rabit.tracks.iter().for_each(|track| {
                    if track.date == *date_str {
                        csv_string = format!(
                            "{}\n{},{},{},",
                            csv_string, date_str, rabit.name, track.value
                        );
                        added_track_for_date = true;
                    }
                });
            });

            if !added_track_for_date {
                csv_string = format!("{}\n{},{},{},", csv_string, date_str, "", "",);
            }
        });

        csv_string = format!("{}{}", "Date,Rabit,Value,", csv_string);

        if let Ok(_) = export_data_to_file(filename, csv_string.as_bytes()) {
            println!("Successfully exported rabit data to rabit_export.csv");
        } else {
            eprintln!("Error exporting rabit data, please try again.");
        }
    }
}

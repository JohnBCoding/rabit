mod cli;
mod rabit;

mod prelude {
    pub use crate::cli::*;
    pub use crate::rabit::*;
    pub use chrono::{Datelike, Days, Local, NaiveDate};
    pub use clap::{Parser, Subcommand};
    pub use serde::{Deserialize, Serialize};
    pub use std::cmp::max;
    pub use std::fs::OpenOptions;
    pub use std::io::Write;
}

use prelude::*;

fn write_data(data: &Data) -> std::io::Result<()> {
    let mut data_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("fluffle.json")?;

    let data_str = serde_json::to_string(data)?;
    data_file.write_all(data_str.as_bytes())?;
    data_file.flush()?;
    Ok(())
}

fn get_data() -> Result<Data, Box<dyn std::error::Error>> {
    let data_file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("fluffle.json")?;

    if data_file.metadata()?.len() == 0 {
        let default_data_file = Data::default();
        Ok(default_data_file)
    } else {
        let data: Data = serde_json::from_reader(data_file)?;
        Ok(data)
    }
}

fn reset_data() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("fluffle.json")?;

    let default_data = Data::default();
    let default_data_str = serde_json::to_string(&default_data)?;
    data_file.write_all(default_data_str.as_bytes())?;
    data_file.flush()?;

    eprintln!("Rabit data fully reset.");

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Ok(mut data) = get_data() {
        match &cli.command {
            Some(Commands::Track {
                name,
                value,
                backtrack,
            }) => {
                let new_rabit = Rabit::new(&data.config, name, value);
                data.track(new_rabit, value, *backtrack);
                let _ = write_data(&data);
                data.print_fluffle(&None, &None);
            }
            Some(Commands::Cull { name, full }) => {
                if *full {
                    let _ = reset_data();
                } else {
                    if let Some(name) = name {
                        data.cull_rabit(&name);
                        let _ = write_data(&data);
                    }
                }
            }
            Some(Commands::Observe {
                name,
                group,
                duration,
            }) => {
                if let Some(name) = name {
                    data.print_rabit(name, group, duration);
                } else {
                    data.print_fluffle(group, duration);
                }
            }
            Some(Commands::Config {
                observe_after_track,
                text_width,
            }) => {
                if let Some(observe_after_track) = observe_after_track {
                    data.config.observe_after_track = *observe_after_track;
                    let _ = write_data(&data);
                }

                if let Some(text_width) = text_width {
                    data.config.view_text_width = *text_width;
                    let _ = write_data(&data);
                }
            }
            None => {}
        }
    } else {
        match &cli.command {
            Some(Commands::Cull { name, full }) => {
                if *full {
                    let _ = reset_data();
                }
            }

            _ => {
                eprintln!("Unable to get data file.");
                eprintln!("If issue persists, try `rabit cull -f` to reset your data file.");
            }
        }
    }
}

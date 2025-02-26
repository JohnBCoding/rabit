mod cli;
mod rabit;

mod prelude {
    pub use crate::cli::*;
    pub use crate::rabit::*;
    pub use clap::{Parser, Subcommand};
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;
    pub use std::fs::OpenOptions;
    pub use std::io::Write;
}

use prelude::*;

fn write_data(data: &Data) -> std::io::Result<()> {
    let mut data_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("data.json")?;

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
        .open("data.json")?;

    if data_file.metadata()?.len() == 0 {
        let default_data_file = Data::default();
        Ok(default_data_file)
    } else {
        let data: Data = serde_json::from_reader(data_file)?;
        Ok(data)
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Track { name }) => {
            if let Ok(mut data) = get_data() {
                let new_rabit = Rabit::new(name);
                data.track(new_rabit);
                let _ = write_data(&data);
                println!("{:?}", data);
            } else {
                eprintln!("Unable to get data file. Exiting..");
            }
        }
        None => {}
    }
}

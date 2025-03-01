use crate::prelude::*;

#[derive(Parser)]
#[command(version, about="A simple habit CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Track a Rabit
    Track {
        /// Name of Rabit to track
        name: String,

        /// Enable value tracking
        #[arg(short, long)]
        value: Option<String>,

        /// Overwrite track for the current day
        #[arg(short, long)]
        backtrack: bool,
    },
    /// Cull A Rabit
    Cull {
        /// Name of Rabit to cull
        name: Option<String>,

        /// Cull entire Fluffle (reset data)
        #[arg(short)]
        full: bool,
    },
    /// View Rabit(s)
    Observe {
        /// Name of Rabit to view
        name: Option<String>,

        /// Sets how observation is grouped, options are 'day' or 'month'
        #[arg(short, long)]
        group: Option<String>,

        /// Sets duration of observation, note that the scale of this is determined by the group type set.
        /// Defaults to 7(duration) days(group) for full rabit observation or 1(duration) month(group) for specific rabit observation
        #[arg(short, long)]
        duration: Option<i32>,
    },

    /// Export your data to csv
    Migrate {
        /// Name of exported file
        name: String,

        ///  Number of days to export, default is 30
        #[arg(short, long)]
        duration: Option<i32>,
    },

    /// Configure CLI Options
    Config {
        /// Toggles print out of rabits after tracking a new rabit
        #[arg(long)]
        observe_after_track: Option<bool>,

        /// Sets text width
        #[arg(short, long)]
        text_width: Option<usize>,

        /// Sets default group type
        group: Option<String>,

        /// Sets default day duration
        #[arg(long)]
        day_duration: Option<i32>,

        /// Sets default month duration
        #[arg(long)]
        month_duration: Option<i32>,

        /// Sets default migrate duration
        #[arg(long)]
        migrate_duration: Option<i32>,
    },
}

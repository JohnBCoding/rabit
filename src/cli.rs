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

        /// Sets duration of observation
        #[arg(short, long)]
        duration: Option<i32>,

        /// Sets how observation is grouped, options are week, month, year
        #[arg(short, long)]
        group: Option<String>,
    },
    /// Configure CLI Options
    Config {
        /// Toggles print out of rabits after tracking a new rabit
        #[arg(long)]
        observe_after_track: Option<bool>,

        /// Sets text width
        #[arg(short, long)]
        text_width: Option<usize>,
    },
}

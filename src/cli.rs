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
    View {
        /// Name of Rabit to view
        name: Option<String>,
    },
    /// Configure CLI Options
    Config {
        #[arg(short, long)]
        text_width: Option<usize>,
    },
}

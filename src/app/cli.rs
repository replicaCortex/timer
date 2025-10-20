pub use clap::Parser;

use super::mode::Mode;

#[derive(Parser, Debug)]
#[command(version="0.0.1", author="replicaCortex",about ="Simple timer", long_about = None)]
pub struct Cli {
    #[arg(short = 'm', long, default_value_t = Mode::Timer)]
    pub mode: Mode,

    /// Set the timer in minutes
    #[arg(short = 'd', long, default_value_t = 5)]
    pub duration: u16,

    /// Notification Title
    #[arg(short = 's', long, default_value_t = String::from(""))]
    pub summary: String,

    /// Main text of the notification
    #[arg(short = 'b', long, default_value_t = String::from(""))]
    pub body: String,
}

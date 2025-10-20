pub use clap::Parser;

use super::mode::Mode;

#[derive(Parser, Debug)]
#[command(version="0.0.2", author="replicaCortex",about ="Simple timer", long_about = None)]
pub struct Cli {
    #[arg(short = 'm', long, default_value_t = Mode::Timer)]
    pub mode: Mode,

    /// Set timer
    #[arg(short = 'd', long, default_value_t = String::from("5m"))]
    pub duration: String,

    /// Notification Title
    #[arg(short = 's', long, default_value_t = String::from(""))]
    pub summary: String,

    /// Main text of the notification
    #[arg(short = 'b', long, default_value_t = String::from(""))]
    pub body: String,
}

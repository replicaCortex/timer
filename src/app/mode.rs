use clap::ValueEnum;
use std::fmt;

#[derive(Default, Debug, ValueEnum, Clone)]
pub enum Mode {
    #[default]
    Timer,
    Alarm,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}

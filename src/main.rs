mod app;
use app::{init_app_and_terminal, init_thread};

use color_eyre::eyre::{Ok, Result};

fn main() -> Result<()> {
    let (app, terminal) = init_app_and_terminal();
    let rc = init_thread();

    app.run(terminal, rc)?;

    ratatui::restore();
    Ok(())
}

use std::io;
use crate::prelude::*;

use ratatui::{Terminal, prelude::CrosstermBackend};


pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {

    // let stdout = io::stdout().into_raw_mode()?;

    Ok(())
}

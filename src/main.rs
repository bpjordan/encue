
mod error;
mod app;
mod prelude;
mod script;
mod logging;

use std::io;

use crossterm::{terminal, ExecutableCommand};
use ratatui::prelude::*;
use crate::prelude::*;

fn main() -> Result<()> {
    let script = script::load()?
        .validate()?;

    let mut term = setup_terminal()?;

    if let Err(e) = app::run(&mut term, &script) {
        eprintln!("{e}")
    }

    restore_terminal(&mut term)
}

fn setup_terminal() -> Result<Terminal<TerminalBackend>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;

    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<TerminalBackend>) -> Result<()> {

    terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;

    Ok(())
}

use ratatui::Terminal;

use crate::{prelude::*, script::Script};

mod render;

pub fn run(terminal: &mut Terminal<TerminalBackend>, show: &Script) -> Result<()> {

    render::render_loop(terminal, show)?;

    Ok(())
}


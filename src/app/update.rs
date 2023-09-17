use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::Backend, Terminal};

use super::{events::Event, render::render, AppState};

use crate::prelude::*;

pub fn update<B: Backend>(event: Event, term: &mut Terminal<B>, app: &mut AppState) -> Result<()> {
    match event {
        Event::Tick => app.upkeep(),
        Event::Resize(_, _) => {}
        Event::Key(k) => handle_key(k, app)?,
        Event::Error(e) => return Err(e.into()),
    }

    render(term, app)
}

pub fn handle_key(key: KeyEvent, app: &mut AppState) -> Result<()> {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Down | KeyCode::Char('j') => app.select_next(),
        KeyCode::Up | KeyCode::Char('k') => app.select_prev(),
        KeyCode::Char(' ') => {
            app.execute_selected()
                .unwrap_or_else(|e| log::error!("Error executing cue: {e}"));
            app.select_next()
        }
        KeyCode::Char('s') => Ok(app.stop_all()),
        _ => Ok(()),
    }
}

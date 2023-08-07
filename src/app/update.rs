use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{Terminal, prelude::Backend};

use super::{AppState, events::Event, render::render};

use crate::prelude::*;

pub fn update<B: Backend>(event: Event, term: &mut Terminal<B>, app: &mut AppState) -> Result<()> {

    match event {
        Event::Tick | Event::Resize(_, _) => {},
        Event::Key(k) => handle_key(k, app)?,
        Event::Error(e) => return Err(e.into()),
    }

    render(term, app)
}

pub fn handle_key(key: KeyEvent, app: &mut AppState) -> Result<()> {

    log::trace!("Main thread got key {key:?}");
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Down
            | KeyCode::Char('j') => app.select_next(),
        KeyCode::Up
            | KeyCode::Char('k') => app.select_prev(),
        _ => Ok(())
    }
}

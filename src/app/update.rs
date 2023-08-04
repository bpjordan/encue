use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{Terminal, prelude::Backend};

use super::{AppState, events::Event, render::render};

use crate::prelude::*;

pub fn update<B: Backend>(event: Event, term: &mut Terminal<B>, app: &mut AppState) -> Result<()> {

    match event {
        Event::Tick => render(term, app),
        Event::Key(k) => handle_key(k, app),
        Event::Resize(_x, _y) => Ok(()),
        Event::Error(e) => Err(e.into()),
    }
}

pub fn handle_key(key: KeyEvent, app: &mut AppState) -> Result<()> {

    log::trace!("Main thread got key {key:?}");
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Down
            | KeyCode::Char('k') => app.select_next(),
        _ => Ok(())
    }
}

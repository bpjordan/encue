
mod error;
mod app;
mod terminal;
mod prelude;
mod cues;
mod logging;
mod util;
mod sound;

use app::{AppState, events::EventListener, update::update};
use cues::Script;

use crate::prelude::*;

fn main() -> Result<()> {
    let script = Script::load()?
        .validate()?;

    let mut app = AppState::new(&script)?;

    let events = EventListener::init();

    let mut term = terminal::setup_terminal()?;

    let rc = loop {
        if !app.active() { break Ok(()) };
        let event = match events.next() {
            Ok(e) => e,
            Err(e) => break Err(e),
        };

        if let Err(e) = update(event, &mut term, &mut app) {
            break Err(e);
        };
    };

    terminal::restore_terminal(term)?;

    rc
}


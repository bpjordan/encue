mod app;
mod cues;
mod error;
mod logging;
mod prelude;
mod sound;
mod terminal;
mod util;

use app::{events::EventListener, update::update, AppState};
use cues::Script;

use crate::prelude::*;

fn main() -> Result<()> {
    let script = Script::load()?.validate()?;

    let mut app = AppState::new(&script)?;

    let events = EventListener::init();

    let mut term = terminal::setup_terminal()?;

    let rc = loop {
        if !app.active() {
            break Ok(());
        };
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

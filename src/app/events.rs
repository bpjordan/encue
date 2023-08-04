use std::{sync::mpsc, thread, time::{Duration, Instant}, io};

use crossterm::event::{KeyEvent, self};

use crate::prelude::*;

const TICKRATE: Duration = Duration::from_millis(100);

#[derive(Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
    Error(io::Error),
}

pub struct EventListener {
    rx: mpsc::Receiver<Event>,
    handler: thread::JoinHandle<Result<()>>,
}

impl EventListener {
    pub fn init() -> Self {

        let (tx, rx) = mpsc::channel();

        let handler = thread::spawn(move || {
            let mut last_tick = Instant::now();

            loop {
                let timeout = TICKRATE.checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::ZERO);

                let ready = match event::poll(timeout) {
                    Ok(r) => r,
                    Err(e) => {
                        tx.send(Event::Error(e))?;
                        break
                    }
                };

                if ready {
                    let event = event::read();
                    match event {
                        Ok(event::Event::Key(k)) => {
                            tx.send(Event::Key(k))?;
                        },
                        Ok(event::Event::Resize(r, c)) => {
                            tx.send(Event::Resize(r, c))?;
                        },
                        Err(e) => {
                            log::trace!("event handler thread encountered error reading event: {e:?}");
                            tx.send(Event::Error(e))?;
                            break
                        },
                        _ => continue
                    }
                } else {
                    tx.send(Event::Tick)?;
                    last_tick = Instant::now();
                };
            }

            Ok(())
        });

        Self { rx, handler }

    }

    pub fn next(&self) -> Result<Event> {
        if self.handler.is_finished() {
            return Err(FatalError::ThreadPanic)
        }
        Ok(self.rx.recv()?)
    }
}

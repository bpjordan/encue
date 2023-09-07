use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log::LevelFilter;
use ratatui::widgets::{Table, TableState};

use crate::logging::{TuiLoggerState, TuiLogger};

use crate::prelude::*;
use crate::cues::Script;
use crate::sound::ExecuteCue;

use super::widgets::cue_list;

pub struct AppState<'a> {
    active: bool,
    widget: Table<'a>,
    cuelist: Vec<&'a str>,
    executables: HashMap<&'a str, Box<dyn ExecuteCue>>,
    list_state: TableState,
    logger_state: Arc<Mutex<TuiLoggerState>>,
}

impl<'a> AppState<'a> {
    pub fn new(script: &'a Script) -> Result<Self> {

        let executables = script.cuelist().into_iter().filter_map(|cue| {
            let label = cue.label();
            match cue.action().prepare(Some(label)) {
                Ok(exe) => {
                    Some((label, exe))
                },
                Err(e) => {
                    log::error!("Error preparing cue `{label}`: {e}");
                    None
                },
            }
        }).collect();

        Ok(Self {
            active: true,
            widget: cue_list(script.cuelist()),
            cuelist: script.cue_names(),
            executables,
            list_state: TableState::default().with_selected(Some(0)),
            logger_state: TuiLogger::init(LevelFilter::Trace)?,
        })
    }

    pub fn quit(&mut self) -> Result<()> {
        log::info!("Quitting application");
        self.active = false;

        Ok(())
    }

}

impl<'a> AppState<'a> {

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn list_state_mut(&mut self) -> &mut TableState {
        &mut self.list_state
    }

    pub fn logger_state(&self) -> &Arc<Mutex<TuiLoggerState>> {
        &self.logger_state
    }

    pub fn widget(&self) -> &Table<'a> {
        &self.widget
    }

}

impl AppState<'_> {
    pub fn select_next(&mut self) -> Result<()> {
        let i = match self.list_state_mut().selected() {
            Some(t) if t < self.cuelist.len() - 1 => {
                t + 1
            }

            _ => 0,
        };

        self.list_state_mut().select(Some(i));

        Ok(())
    }
    
    pub fn select_prev(&mut self) -> Result<()> {
        let i = match self.list_state_mut().selected() {
            Some(t) if t > 0 => {
                t - 1
            }

            _ => self.cuelist.len() - 1,
        };

        self.list_state_mut().select(Some(i));

        Ok(())
    }
}

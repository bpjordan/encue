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
    cuelist: Table<'a>,
    executables: HashMap<String, Box<dyn ExecuteCue>>,
    list_state: TableState,
    logger_state: Arc<Mutex<TuiLoggerState>>,
    cuelist_len: usize,
}

impl<'a> AppState<'a> {
    pub fn new(script: &'a Script) -> Result<Self> {

        Ok(Self {
            active: true,
            cuelist: cue_list(script.cuelist()),
            list_state: TableState::default().with_selected(Some(0)),
            logger_state: TuiLogger::init(LevelFilter::Trace)?,
            cuelist_len: script.cuelist().len(),
            executables: HashMap::new(),
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

    pub fn cuelist(&self) -> &Table<'a> {
        &self.cuelist
    }
}

impl AppState<'_> {
    pub fn select_next(&mut self) -> Result<()> {
        let i = match self.list_state_mut().selected() {
            Some(t) if t < self.cuelist_len - 1 => {
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

            _ => self.cuelist_len - 1,
        };

        self.list_state_mut().select(Some(i));

        Ok(())
    }
}

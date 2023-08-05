use std::sync::{Arc, Mutex};

use log::LevelFilter;
use ratatui::widgets::{ListState, List};

use crate::logging::{TuiLoggerState, TuiLogger};

use crate::prelude::*;
use crate::script::Script;

use super::widgets::cue_list;

pub struct AppState<'a> {
    active: bool,
    cuelist: List<'a>,
    list_state: ListState,
    logger_state: Arc<Mutex<TuiLoggerState>>,
}

impl<'a> AppState<'a> {
    pub fn new(script: &'a Script) -> Result<Self> {

        Ok(Self {
            active: true,
            cuelist: cue_list(script.cuelist()),
            list_state: ListState::default().with_selected(Some(0)),
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

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn logger_state(&self) -> &Arc<Mutex<TuiLoggerState>> {
        &self.logger_state
    }

    pub fn cuelist(&self) -> &List<'a> {
        &self.cuelist
    }
}

impl AppState<'_> {
    pub fn select_next(&mut self) -> Result<()> {
        let i = match self.list_state_mut().selected() {
            Some(t) if t < self.cuelist().len() - 1 => {
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

            _ => self.cuelist().len() - 1,
        };

        self.list_state_mut().select(Some(i));

        Ok(())
    }
}

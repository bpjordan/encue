use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::util::defaults;

use super::Cue;

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Script {
    cuelist: Vec<Cue>,

    #[serde(default = "defaults::default_vol")]
    master: u8,
}

#[allow(dead_code)]
impl Script {

    pub fn cuelist(&self) -> &[Cue] {
        self.cuelist.as_ref()
    }

    pub fn master(&self) -> u8 {
        self.master
    }

    pub fn cue_names(&self) -> Vec<&str> {
        self.cuelist()
            .iter()
            .map(Cue::label)
            .collect()
    }
}

impl Script {

    pub fn load() -> Result<Self> {
        let f = fs::File::open("script.yaml")?;
        let yaml = io::read_to_string(f)?;

        Ok(serde_yaml::from_str(&yaml)?)
    }

    pub fn new(cues: Vec<Cue>) -> Self {
        Self {
            cuelist: cues,
            master: 100
        }
    }

    pub fn with_master(mut self, master: u8) -> Self {
        self.master = master;
        self
    }

}


use std::{
    time::Duration,
    str::FromStr,
    convert::Infallible
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::util::defaults;


#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FadeCue {
    target: String,

    #[serde(default)]
    volume: u8,

    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    #[serde(default = "defaults::default_fade_duration")]
    duration: Duration
}

impl FadeCue {
    pub fn target(&self) -> &str {
        self.target.as_ref()
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }
}

impl FadeCue {
    pub fn new(target: impl ToString) -> Self {
        Self {
            target: target.to_string(),
            volume: Default::default(),
            duration: defaults::default_fade_duration(),
        }
    }

    pub fn to_volume(mut self, volume: u8) -> Self {
        self.volume = volume;
        self
    }

    pub fn for_duration(mut self, duration: impl Into<Duration>) -> Self {
        self.duration = duration.into();
        self
    }
}

impl FromStr for FadeCue {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

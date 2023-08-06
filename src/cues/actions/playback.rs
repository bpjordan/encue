use std::{
    path::{PathBuf, Path},
    str::FromStr,
    convert::Infallible, time::Duration
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaybackCue {
    file: PathBuf,

    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac>")]
    duration: Option<Duration>,

    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac>")]
    fade_in: Option<Duration>,

    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac>")]
    fade_out: Option<Duration>
}

impl PlaybackCue {

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn fade_in(&self) -> Option<Duration> {
        self.fade_in
    }

    pub fn fade_out(&self) -> Option<Duration> {
        self.fade_out
    }
}

impl PlaybackCue {
    pub fn new(file: impl AsRef<Path>) -> Self {
        Self {
            file: file.as_ref().to_path_buf(),
            duration: None,
            fade_in: None,
            fade_out: None,
        }
    }

    pub fn for_duration(mut self, duration: impl Into<Duration>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    pub fn fade_in_for(mut self, duration: impl Into<Duration>) -> Self {
        self.fade_in = Some(duration.into());
        self
    }

    pub fn fade_out_for(mut self, duration: impl Into<Duration>) -> Self {
        self.fade_out = Some(duration.into());
        self
    }
}

impl FromStr for PlaybackCue {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

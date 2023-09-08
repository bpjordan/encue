use std::{
    path::{PathBuf, Path},
    str::FromStr,
    convert::Infallible, time::Duration, io::{self, BufReader}
};

use rodio::{Source, Decoder, Sink};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;

use crate::sound::{PrepareCue, PlaybackExecutable};

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

#[allow(dead_code)]
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

#[derive(Debug, Error)]
pub enum PlaybackCueError {
    #[error(transparent)]
    File(#[from] io::Error),

    #[error("Decoder error: {0}")]
    Decode(#[from] rodio::decoder::DecoderError)
}

impl PrepareCue for PlaybackCue {
    type Executable = PlaybackExecutable;

    type PrepareError = PlaybackCueError;

    fn prepare(&self, label: Option<&str>) -> Result<Self::Executable, Self::PrepareError> {
        let f = std::fs::File::open(self.file())?;

        let mut s: Box<dyn Source<Item = i16> + Send + Sync> = Box::new(Decoder::new(BufReader::new(f))?);

        if let Some(d) = self.duration() {
            s = Box::new(s.take_duration(d));
        }

        if let Some(d) = self.fade_in() {
            s = Box::new(s.fade_in(d))
        }

        let (sink, queue) = Sink::new_idle();

        sink.append(s);

        Ok(PlaybackExecutable::new(label.map(ToString::to_string), sink, queue))
    }
}

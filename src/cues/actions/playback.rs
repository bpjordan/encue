use std::{
    convert::Infallible,
    io::{self, BufReader, Seek},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use lofty::AudioFile;
use rodio::{Decoder, Sink, Source};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;

use crate::sound::{metadata::PlaybackMeta, PlaybackExecutable, PrepareCue};

#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaybackCue {
    file: PathBuf,

    volume: Option<u8>,

    #[serde(default)]
    #[serde(alias = "loop")]
    repeat: bool,

    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac>")]
    duration: Option<Duration>,

    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac>")]
    fade_in: Option<Duration>,

    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac>")]
    fade_out: Option<Duration>,
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
            repeat: false,
            volume: None,
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
    Decode(#[from] rodio::decoder::DecoderError),

    #[error("Error retrieving metadata: {0}")]
    Metadata(#[from] lofty::LoftyError),
}

impl PrepareCue for PlaybackCue {
    type Executable = PlaybackExecutable;

    type PrepareError = PlaybackCueError;

    fn prepare(&self, label: Option<&str>) -> Result<Self::Executable, Self::PrepareError> {
        let mut f = std::fs::File::open(self.file())?;

        let duration = match self.duration() {
            Some(d) => d,
            None => lofty::read_from(&mut f)?.properties().duration(),
        };

        f.seek(io::SeekFrom::Start(0))?;

        let mut s: Box<dyn Source<Item = i16> + Send + Sync> =
            Box::new(Decoder::new(BufReader::new(f))?.take_duration(duration));

        let meta = Arc::new(Mutex::new(PlaybackMeta {
            start: Instant::now(),
            duration,
        }));

        if self.repeat {
            s = Box::new(s.repeat_infinite())
        }

        if let Some(d) = self.fade_in() {
            s = Box::new(s.fade_in(d))
        }

        let (sink, queue) = Sink::new_idle();

        // TODO: This step causes a panic in some circumstances. Need to investigate further
        let meta_mut = meta.clone();
        s = Box::new(s.periodic_access(duration, move |_| {
            meta_mut.lock().unwrap().start = Instant::now();
        }));

        if let Some(fade_duration) = self.fade_out() {
            let start_duration = duration.saturating_sub(fade_duration);

            let s = s.buffered();
            let start = s.clone().take_duration(start_duration);
            let mut end = s.skip_duration(start_duration).take_duration(fade_duration);
            end.set_filter_fadeout();

            sink.append(start);
            sink.append(end);
        } else {
            sink.append(s);
        }

        if let Some(vol) = self.volume {
            sink.set_volume(vol as f32 / 100.0)
        }

        Ok(PlaybackExecutable::new(
            label.map(ToString::to_string),
            sink,
            queue,
            meta,
        ))
    }
}

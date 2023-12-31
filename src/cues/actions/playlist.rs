use std::{
    convert::Infallible,
    io::{self, BufReader, Seek},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use itertools::Either;
use lofty::AudioFile;
use rand::{seq::SliceRandom, thread_rng};
use rodio::{source, Decoder, Sink};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;

use crate::sound::{metadata::PlaybackMeta, PlaybackExecutable, PrepareCue};

#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PlaylistCue {
    folder: Option<PathBuf>,

    #[serde(default)]
    files: Vec<PathBuf>,

    #[serde(default)]
    #[serde(alias = "loop")]
    repeat: bool,

    #[serde(default)]
    shuffle: bool,

    volume: Option<u8>,

    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac>")]
    crossfade: Option<Duration>,
}

#[allow(dead_code)]
impl PlaylistCue {
    pub fn from_folder(folder: PathBuf) -> Self {
        Self {
            folder: Some(folder),
            files: Vec::new(),
            ..Default::default()
        }
    }

    pub fn from_files(files: Vec<PathBuf>) -> Self {
        Self {
            folder: None,
            files,
            ..Default::default()
        }
    }

    pub fn with_file(mut self, file: PathBuf) -> Self {
        self.files.push(file);
        self
    }

    pub fn folder(&self) -> Option<&Path> {
        self.folder.as_ref().map(AsRef::as_ref)
    }

    pub fn with_repeat(mut self) -> Self {
        self.repeat = true;
        self
    }

    pub fn with_shuffle(mut self) -> Self {
        self.shuffle = true;
        self
    }
}

impl FromStr for PlaylistCue {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_folder(PathBuf::from(s)))
    }
}

#[derive(Debug, Error)]
pub enum PlaylistCueError {
    #[error(transparent)]
    File(#[from] io::Error),

    #[error("Decoder error: {0}")]
    Decode(#[from] rodio::decoder::DecoderError),
}

impl PrepareCue for PlaylistCue {
    type Executable = PlaybackExecutable;

    type PrepareError = PlaylistCueError;

    fn prepare(&self, label: Option<&str>) -> Result<Self::Executable, Self::PrepareError> {
        let mut files = self.files.clone();

        if let Some(folder) = self.folder().to_owned() {
            files.extend(
                folder
                    .read_dir()?
                    .filter_map(|entry| Some(entry.ok()?.path())),
            );
        }

        if self.shuffle {
            files.shuffle(&mut thread_rng())
        }

        let meta = Arc::new(Mutex::new(PlaybackMeta {
            start: Instant::now(),
            duration: Duration::ZERO,
        }));

        let meta_update = meta.clone();

        let sources = if self.repeat {
            Either::Left(files.into_iter().cycle())
        } else {
            Either::Right(files.into_iter())
        }
        .filter_map(move |filename| {
            let mut file = match std::fs::File::open(&filename) {
                Err(e) => {
                    log::warn!(
                        "Skipped playlist file {} due to IO error: {e}",
                        filename.display()
                    );
                    return None;
                }
                Ok(f) => f,
            };

            let duration = match lofty::read_from(&mut file) {
                Ok(tags) => tags.properties().duration(),
                Err(e) => {
                    log::warn!(
                        "Skipped playlist file {} due to metadata error: {e}",
                        filename.display()
                    );
                    return None;
                }
            };

            if let Err(e) = file.seek(io::SeekFrom::Start(0)) {
                log::warn!(
                    "Skipped playlist file {} due to Seek error: {e}",
                    filename.display()
                );
                return None;
            };

            let s = match Decoder::new(BufReader::new(file)) {
                Ok(d) => d,
                Err(e) => {
                    log::warn!(
                        "Skipped playing file `{}` due to audio decoding error: {e}",
                        filename.display()
                    );
                    return None;
                }
            };

            *meta_update.lock().unwrap() = PlaybackMeta {
                start: Instant::now(),
                duration,
            };

            log::debug!("Loading playlist file `{}`", filename.display());

            Some(s)
        });

        let s = source::from_iter(sources);

        let (sink, queue) = Sink::new_idle();

        sink.append(s);

        if let Some(vol) = self.volume {
            sink.set_volume(f32::from(vol) / 100.0)
        }

        Ok(PlaybackExecutable::new(
            label.map(ToString::to_string),
            sink,
            queue,
            meta,
        ))
    }
}

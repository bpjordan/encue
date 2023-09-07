use std::{path::{PathBuf, Path}, str::FromStr, convert::Infallible, time::Duration, io::{self, BufReader}};

use rodio::{Sink, Source, Decoder, source};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;

use crate::sound::{PrepareCue, PlaybackExecutable};

#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PlaylistCue {
    folder: Option<PathBuf>,
    files: Vec<PathBuf>,

    #[serde(default)]
    repeat: bool,

    #[serde(default)]
    #[serde(alias = "loop")]
    shuffle: bool,

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
    Decode(#[from] rodio::decoder::DecoderError)
}

impl PrepareCue for PlaylistCue {
    type Executable = PlaybackExecutable;

    type PrepareError = PlaylistCueError;

    fn prepare(&self, label: Option<&str>) -> Result<Self::Executable, Self::PrepareError> {
        let mut files = self.files.clone();

        if let Some(folder) = self.folder().to_owned() {
            files.extend(folder.read_dir()?.filter_map(|entry| {
                Some(entry.ok()?.path())
            }));
        }

        let sources = files.into_iter().filter_map(|filename| {
            let decoder = match std::fs::File::open(&filename) {
                Err(e) => {
                    log::warn!("Skipped file {} due to IO error: {e}", filename.display());
                    return None
                },
                Ok(f) => Decoder::new(BufReader::new(f)),
            };

            let s: Box<dyn Source<Item = i16> + Send + Sync> = match decoder {
                Ok(d) => Box::new(d),
                Err(e) => {
                    log::warn!("Skipped loading file {} due to audio decoding error: {e}", filename.display());
                    return None
                },
            };

            Some(s)
        }).collect::<Vec<_>>();

        let mut s: Box<dyn Source<Item = i16> + Send + Sync> = Box::new(source::from_iter(sources));

        if self.repeat {
            s = Box::new(s.repeat_infinite())
        }

        let (sink, queue) = Sink::new_idle();

        sink.append(s);

        Ok(PlaybackExecutable::new(label.map(ToString::to_string), sink, queue))
    }
}


use std::{path::PathBuf, str::FromStr, convert::Infallible, time::Duration};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PlaylistCue {
    folder: Option<PathBuf>,
    files: Vec<PathBuf>,

    #[serde(default)]
    repeat: bool,

    #[serde(default)]
    shuffle: bool,

    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac>")]
    crossfade: Option<Duration>,
}

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

    pub fn folder(&self) -> Option<&PathBuf> {
        self.folder.as_ref()
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

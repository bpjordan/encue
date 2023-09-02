use std::{error::Error, convert::Infallible, io};

use rodio::PlayError;
use thiserror::Error;

use super::AudioEngine;

#[derive(Debug, Error)]
pub enum ExecuteCueError {

    #[error("Couldn't find target cue {0}")]
    MissingTarget(String),

    #[error(transparent)]
    IO(io::Error),

    #[error(transparent)]
    Playback(#[from] PlayError),

    #[error("{0}")]
    General(&'static str)
}

pub trait ExecuteCue {
    fn execute(self, engine: &mut AudioEngine) -> Result<(), ExecuteCueError>;
}

pub trait PrepareCue {
    type Executable: ExecuteCue;
    type PrepareError: Error + Send + Sync;

    fn prepare(&self, label: Option<&str>) -> Result<Self::Executable, Self::PrepareError>;

    fn validate(&self, label: Option<&str>) -> Result<(), Self::PrepareError> {
        self.prepare(label).and(Ok(()))
    }
}

impl<T: ExecuteCue + Clone> PrepareCue for T {
    type Executable = T;
    type PrepareError = Infallible;

    fn prepare(&self, _: Option<&str>) -> Result<Self::Executable, Self::PrepareError> {
        Ok(self.clone())
    }
}
